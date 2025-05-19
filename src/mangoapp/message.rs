use std::time::Duration;

use packed_struct::prelude::*;

pub const MSG_HEADER_SIZE: usize = 12;
pub const MSG_V1_SIZE: usize = 85;

/// The header contains the message type and version of the message so it can
/// be properly unpacked.
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq)]
#[packed_struct(bit_numbering = "msb0", size_bytes = "12")]
pub struct MangoAppMsgHeader {
    #[packed_field(bytes = "0..=7", endian = "lsb")]
    msg_type: i64,
    #[packed_field(bytes = "8..=11", endian = "lsb")]
    version: u32,
}

impl MangoAppMsgHeader {
    /// Returns the version of the MangoApp message
    pub fn version(&self) -> u32 {
        self.version
    }
}

/// Gamescope frame information designed to be sent to MangoApp.
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq)]
#[packed_struct(bit_numbering = "msb0", size_bytes = "85")]
pub struct MangoAppMsgV1 {
    #[packed_field(bytes = "0..=11")]
    header: MangoAppMsgHeader,
    #[packed_field(bytes = "12..=15", endian = "lsb")]
    pid: u32,
    #[packed_field(bytes = "16..=23", endian = "lsb")]
    app_frametime_ns: u64,
    #[packed_field(bytes = "24")]
    fsr_upscale: u8,
    #[packed_field(bytes = "25")]
    fsr_sharpness: u8,
    #[packed_field(bytes = "26..=33", endian = "lsb")]
    visible_frametime_ns: u64,
    #[packed_field(bytes = "34..=41", endian = "lsb")]
    latency_ns: u64,
    #[packed_field(bytes = "42..=45", endian = "lsb")]
    output_width: u32,
    #[packed_field(bytes = "46..=49", endian = "lsb")]
    output_height: u32,
    #[packed_field(bytes = "50..=51", endian = "lsb")]
    display_refresh: u16,
    #[packed_field(bits = "423")]
    app_wants_hdr: bool,
    #[packed_field(bits = "422")]
    steam_focused: bool,
    #[packed_field(bytes = "53..=84")]
    engine_name: [u8; 32],
}

impl MangoAppMsgV1 {
    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn app_frametime(&self) -> Duration {
        Duration::from_nanos(self.app_frametime_ns)
    }

    pub fn fsr_upscale(&self) -> u8 {
        self.fsr_upscale
    }

    pub fn fsr_sharpness(&self) -> u8 {
        self.fsr_sharpness
    }

    pub fn visible_frametime(&self) -> Duration {
        Duration::from_nanos(self.visible_frametime_ns)
    }

    pub fn latency(&self) -> Duration {
        Duration::from_nanos(self.latency_ns)
    }

    pub fn output_width(&self) -> u32 {
        self.output_width
    }

    pub fn output_height(&self) -> u32 {
        self.output_height
    }

    pub fn display_refresh(&self) -> u16 {
        self.display_refresh
    }

    pub fn app_wants_hdr(&self) -> bool {
        self.app_wants_hdr
    }

    pub fn steam_focused(&self) -> bool {
        self.steam_focused
    }

    pub fn engine_name(&self) -> String {
        std::str::from_utf8(&self.engine_name)
            .map(|v| v.to_string())
            .unwrap_or_default()
    }
}
