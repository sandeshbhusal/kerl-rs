use crate::{
    conf::rtt::{count_up_channels, ENABLED},
    drivers::rtt::_SEGGER_RTT,
};
use core::fmt::Write;
use log::{Log, Metadata, Record};

pub struct RTTLogger {
    channel_idx: usize,
}

impl RTTLogger {
    pub const fn new(channel_idx: usize) -> Self {
        Self { channel_idx }
    }
}

#[allow(dead_code)]
struct RttChannelWriter {
    channel_idx: usize,
}

impl Write for RttChannelWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let rb = unsafe { &mut _SEGGER_RTT.up_chans[self.channel_idx] };
        rb.write(s);
        Ok(())
    }
}

impl Log for RTTLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        ENABLED && self.channel_idx < count_up_channels()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut writer = RttChannelWriter {
                channel_idx: self.channel_idx,
            };

            let _ = write!(writer, "[{}] {}\n", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

// Sotre as static for now.
static RTT_LOGGER: RTTLogger = RTTLogger::new(0);

pub unsafe fn init_logger() {
    let _ = log::set_logger_racy(&RTT_LOGGER);
    log::set_max_level_racy(log::LevelFilter::Debug);
}

pub static LOGGING_BACKEND: &str = "RTT";
