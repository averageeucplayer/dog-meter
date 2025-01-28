
#![allow(unused_variables)]

pub mod packets;
pub mod decryption;

use std::{error::Error, sync::mpsc::{self, Receiver}};

use packets::opcodes::Pkt;

pub type GearLevel = f32;
pub type EntityId = u64;
pub type CharacterId = u64;
pub type NpcId = u32;
pub type SkillId = u32;
pub type SkillEffectId = u32;
pub type ClassId = u32;
pub type PartyInstanceId = u32;
pub type RaidInstanceId = u32;
pub type StatusEffectId = u32;
pub type StatusEffectInstanceId = u32;

pub fn start_capture(_port: u16, _region_file_path: String) -> Result<Receiver<(Pkt, Vec<u8>)>, Box<dyn Error>> {
    let (_tx, rx) = mpsc::channel::<(Pkt, Vec<u8>)>();

    Ok(rx)
}