use std::{error::Error, sync::mpsc::{self, Receiver}, time::Duration};
use meter_core::packets::{definitions::*, opcodes::Pkt};
use tokio::time::sleep;

use crate::fight_simulator::*;

use super::PacketSniffer;

pub struct FakePacketSniffer {

}

impl PacketSniffer for FakePacketSniffer {
    fn start_capture(&self, port: u16, region_file_path: String) -> Result<Receiver<(Pkt, Vec<u8>)>, Box<dyn Error>> {
        let (tx, rx) = mpsc::channel::<(Pkt, Vec<u8>)>();

        tokio::spawn(async move {

            let options = FightSimulatorOptions {
                min_dps: 1e8 as i64,
                max_dps: 10e8 as i64,
                min_sup_dps: 1e6 as i64,
                max_sup_dps: 1e7 as i64
            };
            let mut fight_simulator = FightSimulator::new(options);

            fight_simulator.set_boss("Phantom Manifester Brelshaza", 720284908120);

            let packet = fight_simulator.get_local_player_packet();
            tx.send(packet).unwrap();

            let packets = fight_simulator.get_player_packets();
            
            for packet in packets {
                tx.send(packet).unwrap();
            }

            let packet = fight_simulator.get_party_info_packet();
            tx.send(packet).unwrap();

            let packet = fight_simulator.get_boss_packet();
            tx.send(packet).unwrap();

            loop {
            
                let tuple = fight_simulator.get_damage_packet();
                tx.send(tuple).unwrap();
                
                sleep(Duration::from_millis(500)).await;
            }
        });

        Ok(rx)
    }
}

impl FakePacketSniffer {
    pub fn new() -> Self {
        Self {
            
        }
    }
}