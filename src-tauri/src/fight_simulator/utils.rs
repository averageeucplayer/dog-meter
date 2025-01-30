use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::models::*;

use super::class_template::*;

pub fn should_execute(probability: f32) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>() < probability
}

pub fn random_item<T: Clone>(arr: &[T]) -> T {
    let item = arr.choose(&mut thread_rng()).unwrap();
    item.clone()
}

pub fn random_item_mut<T>(arr: &mut [T]) -> &mut T {
    let index = thread_rng().gen_range(0..arr.len());
    &mut arr[index]
}

pub fn get_random_u64(min: u64, max: u64) -> u64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(min..=max)
}

pub fn get_random_u32(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(min..=max)
}

pub fn get_random_i64(min: i64, max: i64) -> i64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(min..=max)
}

pub fn get_random_f32(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(min..=max)
}

pub fn get_random_gear_level(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(min..=max)
}

pub fn get_random_dps_class_template() -> &'static ClassTemplate<'static> {
    let class = DPS_CLASS_TEMPLATES.choose(&mut thread_rng()).unwrap();
    class
}

pub fn get_random_sup_class_template() -> &'static ClassTemplate<'static> {
    let class = SUP_CLASS_TEMPLATES.choose(&mut thread_rng()).unwrap();
    class
}

pub fn calculate_modifier(hit_flag: HitFlag, hit_option: HitOption) -> i32 {
    let hit_flag_value = hit_flag as i32;
    let hit_option_value = hit_option as i32;

    (hit_option_value << 4) | hit_flag_value
}

pub fn random_hit_flag() -> HitFlag {
    let mut rng = rand::thread_rng();
    let random_value = rng.gen_range(0..=13);

    unsafe { std::mem::transmute(random_value as u32) }
}

pub fn random_hit_option() -> HitOption {
    let mut rng = rand::thread_rng();
    let random_value = rng.gen_range(0..=4);

    unsafe { std::mem::transmute(random_value as u8) }
}