use crate::drivers::rtt::{ChannelConfig, ChannelType};

pub const BUFFER_SIZE: u32 = 1024;
pub const ENABLED: bool = true;

pub(crate) const CHANNEL_CONFIGS: [ChannelConfig; 2] = [
    ChannelConfig {
        name: "foo",
        chan_type: ChannelType::Down,
    },
    ChannelConfig {
        name: "mcu",
        chan_type: ChannelType::Up,
    },
];

pub(crate) const fn count_up_channels() -> usize {
    let mut count = 0;
    let mut i = 0;
    while i < CHANNEL_CONFIGS.len() {
        if matches!(CHANNEL_CONFIGS[i].chan_type, ChannelType::Up) {
            count += 1;
        }
        i += 1;
    }
    count
}

pub(crate) const fn count_down_channels() -> usize {
    return CHANNEL_CONFIGS.len() - count_up_channels();
}
