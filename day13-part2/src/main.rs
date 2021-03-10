extern crate num;

use num::bigint::BigInt;
use num::Integer;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let second_line = line_iter.nth(1).unwrap().unwrap();

    let ids: Vec<(usize, u32)> = second_line
        .split(',')
        .enumerate()
        .filter_map(|(i, s)| {
            if let Ok(x) = s.parse::<u32>() {
                Some((i, x))
            } else {
                None
            }
        })
        .collect();

    println!("IDs: {:?}", ids);

    let n1: u64 = 221;
    let n2: u64 = 19;
    let offset: u64 = 3;
    let prev_time: u64 = 102;
    println!(
        "Time for {} and {} with offset {} : {:?}",
        n1,
        n2,
        offset,
        find_time_at_offset(n1, n2, prev_time, offset)
    );
}

// Will be faster if prev_lcm > n2. prev_lcm is intended to be the lcm of previous values
// and prev_time is the previously found time.
fn find_time_at_offset(prev_lcm: u64, n2: u64, prev_time: u64, offset: u64) -> Option<(u64, u64)> {
    let new_lcm: u64 = num::Integer::lcm(&prev_lcm, &n2);

    for time in (prev_lcm..=(new_lcm + prev_time)).rev().step_by(prev_lcm as usize) {
        if (time + 1) % n2 == 0 {
            return Some(((time * offset) % new_lcm, new_lcm));
        }
    }

    None
}
