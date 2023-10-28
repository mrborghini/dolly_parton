use std::ops::RangeInclusive;
use rand::Rng;

pub fn random_number(startrange: usize, maxrange: usize) -> usize {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(RangeInclusive::new(startrange, maxrange));
    return num;
}