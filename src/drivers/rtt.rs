//! Very simple RTT Driver for post-boot debug
#![allow(dead_code)]

use core::ptr::addr_of_mut;

use crate::conf::rtt::{count_down_channels, count_up_channels, BUFFER_SIZE};

pub(crate) enum ChannelType {
    Up,
    Down,
}

pub(crate) struct ChannelConfig {
    pub(crate) name: &'static str,
    pub(crate) chan_type: ChannelType,
}

const UP_CHANNELS_COUNT: usize = count_up_channels();
const DOWN_CHANNELS_COUNT: usize = count_down_channels();

/* Statically allocate the buffers
 * TODO: if this affects binary size (prolly does, .data)
 */
#[no_mangle]
static mut UP_CHANNEL_BUFFERS: [[u8; BUFFER_SIZE as usize]; UP_CHANNELS_COUNT] =
    [[0; BUFFER_SIZE as usize]; UP_CHANNELS_COUNT];

#[no_mangle]
static mut DOWN_CHANNEL_BUFFERS: [[u8; BUFFER_SIZE as usize]; DOWN_CHANNELS_COUNT] =
    [[0; BUFFER_SIZE as usize]; DOWN_CHANNELS_COUNT];

#[repr(C)]
#[derive(Debug)]
pub struct RingBuffer {
    name: *const u8,
    buffer: *mut u8,
    buf_size: u32,
    wr_offset: u32,
    rd_offset: u32,
    flags: u32,
}

impl RingBuffer {
    const fn raw() -> Self {
        Self {
            name: core::ptr::null(),
            buffer: core::ptr::null_mut(),
            buf_size: BUFFER_SIZE,
            wr_offset: 0,
            rd_offset: 0,
            flags: 0,
        }
    }

    // Read volatile the wr offset
    pub fn wr_offset(&self) -> u32 {
        unsafe { core::ptr::read_volatile(core::ptr::addr_of!(self.wr_offset)) }
    }

    // Read volatile the rd offset
    pub fn rd_offset(&self) -> u32 {
        unsafe { core::ptr::read_volatile(core::ptr::addr_of!(self.rd_offset)) }
    }

    // Update the wr offset counter.
    pub fn update_wr_offset(&mut self, wr_offset: u32) {
        unsafe {
            core::ptr::write_volatile(core::ptr::addr_of_mut!(self.wr_offset), wr_offset);
        }
    }

    // Update the rd offset counter
    pub fn update_rd_offset(&mut self, rd_offset: u32) {
        unsafe {
            core::ptr::write_volatile(core::ptr::addr_of_mut!(self.rd_offset), rd_offset);
        }
    }

    pub fn write(&mut self, data: &str) -> usize {
        let data_bytes = data.as_bytes();
        let data_len = data_bytes.len();
        let buf_size = self.buf_size as usize;

        // Load offsets with volatile to ensure we see debugger updates
        let wr_offset = self.wr_offset() as usize;
        let rd_offset = self.rd_offset() as usize;

        // Calculate available space in the ring buffer
        let available = if wr_offset >= rd_offset {
            buf_size - (wr_offset - rd_offset) - 1
        } else {
            rd_offset - wr_offset - 1
        };

        let to_write = core::cmp::min(data_len, available);
        if to_write == 0 {
            return 0;
        }

        let space_to_end = buf_size - wr_offset;
        let first_chunk = core::cmp::min(to_write, space_to_end);

        unsafe {
            // Write first chunk (up to the end of the buffer)
            core::ptr::copy_nonoverlapping(
                data_bytes.as_ptr(),
                self.buffer.add(wr_offset),
                first_chunk,
            );

            // Write second chunk (wrap around to the beginning)
            if to_write > first_chunk {
                core::ptr::copy_nonoverlapping(
                    data_bytes.as_ptr().add(first_chunk),
                    self.buffer,
                    to_write - first_chunk,
                );
            }
        }

        // Update the write offset
        let new_wr_offset = (wr_offset + to_write) % buf_size;
        self.update_wr_offset(new_wr_offset as u32);

        to_write
    }
}

#[repr(C, align(4))]
#[derive(Debug)]
pub struct ControlBlock {
    pub id: [u8; 16],
    pub num_up_chans: u32,
    pub num_down_chans: u32,
    pub up_chans: [RingBuffer; UP_CHANNELS_COUNT],
    pub down_chans: [RingBuffer; DOWN_CHANNELS_COUNT],
}

impl ControlBlock {
    pub const fn new() -> Self {
        let mut cb = ControlBlock {
            id: [
                b'S', b'E', b'G', b'G', b'E', b'R', b' ', b'R', b'T', b'T', b'\0', b'\0', b'\0',
                b'\0', b'\0', b'\0',
            ],
            num_up_chans: UP_CHANNELS_COUNT as u32,
            num_down_chans: DOWN_CHANNELS_COUNT as u32,
            up_chans: [RingBuffer::raw(); UP_CHANNELS_COUNT],
            down_chans: [RingBuffer::raw(); DOWN_CHANNELS_COUNT],
        };

        // TODO: Using For loop in const functions is not stable

        let mut chan_id = 0;
        while chan_id < cb.up_chans.len() {
            let up_chan = &mut cb.up_chans[chan_id];
            up_chan.name = "MCU\0".as_ptr();
            up_chan.buffer = unsafe { addr_of_mut!(UP_CHANNEL_BUFFERS[chan_id]) as *mut u8 };
            chan_id += 1;
        }

        let mut chan_id = 0;
        while chan_id < cb.down_chans.len() {
            let down_chan = &mut cb.down_chans[chan_id];
            down_chan.name = "HOST\0".as_ptr();
            down_chan.buffer = unsafe { DOWN_CHANNEL_BUFFERS[chan_id].as_ptr() as *mut u8 };
            chan_id += 1;
        }

        return cb;
    }
}

#[link_section = ".data.rtt"]
#[no_mangle]
pub static mut _SEGGER_RTT: ControlBlock = ControlBlock::new();
