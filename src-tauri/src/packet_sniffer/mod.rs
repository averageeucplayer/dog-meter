mod fake;
mod windivert;

pub use fake::FakePacketSniffer;
pub use windivert::WindivertPacketSniffer;
use std::{error::Error, sync::mpsc::Receiver};

use meter_core::packets::opcodes::Pkt;

pub trait PacketSniffer {
    fn start_capture(&self, port: u16, region_file_path: String) -> Result<Receiver<(Pkt, Vec<u8>)>, Box<dyn Error>>;
}
