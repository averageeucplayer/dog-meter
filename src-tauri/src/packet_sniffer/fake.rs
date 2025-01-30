use std::{error::Error, panic, sync::mpsc::{self, Receiver}, time::Duration};
use meter_core::packets::{definitions::*, opcodes::Pkt};
use tokio::time::sleep;

use crate::fight_simulator::{utils::random_item, *};

use super::{utils::shuffle_array, PacketSniffer};

pub struct FakePacketSniffer {

}

impl PacketSniffer for FakePacketSniffer {
    fn start_capture(&self, port: u16, region_file_path: String) -> Result<Receiver<(Pkt, Vec<u8>)>, Box<dyn Error>> {
        let (tx, rx) = mpsc::channel::<(Pkt, Vec<u8>)>();

        tokio::spawn(async move {

            let mut player_names = vec!["Molenzwiebel", "Snow", "Baker", "Mathi", "Inngrimsch", "Poont", "Legalia", "Melatonin"];
            let local_player_name = random_item(&player_names);
            shuffle_array(&mut player_names);

            let options = FightSimulatorOptions {
                min_crit_rate: 0.7,
                max_crit_rate: 0.8,
                sup_crit_rate: 0.1,
                min_id: 10000,
                max_id: 100000,
                min_entity_id: 10000,
                max_entity_id: 100000,
                min_ilvl: 1690.0,
                max_ilvl: 1710.0,
                min_player_hp: 2.5e5 as i64,
                max_player_hp: 3e5 as i64,
                min_dps: 1e8 as i64,
                max_dps: 10e8 as i64,
                min_awakening_dps: 1e9 as i64,
                max_awakening_dps: 4e9 as i64,
                min_sup_dps: 1e6 as i64,
                max_sup_dps: 1e7 as i64,
                player_names,
                party_count: 2,
                local_player_name,
                boss_name: "Phantom Manifester Brelshaza",
                boss_max_hp: 720284908120,
                zone_id: 37522,
                zone_level: ZoneLevel::Hard,
                trigger_signal_on_end: 57,
                use_hyper_awakening_after: Duration::from_secs(300),
                perform_counter_every: Some(Duration::from_secs(30)),
                players_dead_after: vec![
                    ("Baker", Duration::from_secs(30))
                ],
            };

            let mut fight_simulator = FightSimulator::new(options);

            let packet = fight_simulator.get_local_player_packet();
            tx.send(packet).unwrap();

            let packets = fight_simulator.get_player_packets();
            
            for packet in packets {
                tx.send(packet).unwrap();
            }

            let packets = fight_simulator.get_party_info_packet();

            for packet in packets {
                tx.send(packet).unwrap();
            }

            let packet = fight_simulator.get_boss_packet();
            tx.send(packet).unwrap();

            let packet = fight_simulator.get_zone_member_load_packet();
            tx.send(packet).unwrap();

            loop {
                fight_simulator.update_time();

                if fight_simulator.is_boss_dead() {
                    
                    let packet = fight_simulator.get_trigger_start_packet();
                    tx.send(packet).unwrap();
                }
                else {
                    let packets = fight_simulator.perform_special_actions();

                    for packet in packets {
                        tx.send(packet).unwrap();
                    }

                    let packet = fight_simulator.random_player_damage_packet();
                    tx.send(packet).unwrap();
                }
                
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