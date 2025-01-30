use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn shuffle_array<T>(arr: &mut [T]) {
    arr.shuffle(&mut thread_rng());
}