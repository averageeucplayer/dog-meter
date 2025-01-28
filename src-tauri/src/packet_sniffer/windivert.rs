use std::{error::Error, sync::mpsc::Receiver};
use meter_core::packets::opcodes::Pkt;

use super::PacketSniffer;

pub struct WindivertPacketSniffer {
    
}

impl PacketSniffer for WindivertPacketSniffer {
    fn start_capture(&self, port: u16, region_file_path: String) -> Result<Receiver<(Pkt, Vec<u8>)>, Box<dyn Error>> {
        meter_core::start_capture(port, region_file_path)
    }
}

impl WindivertPacketSniffer {
    pub fn new() -> Self {
        Self {}
    }
}

