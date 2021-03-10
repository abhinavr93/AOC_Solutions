extern crate num;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let second_line = line_iter.nth(1).unwrap().unwrap();

    let mut ids_iter = second_line.split(',').enumerate().filter_map(|(i, s)| {
        if let Ok(x) = s.parse::<u32>() {
            Some((i, x))
        } else {
            None
        }
    });

    //println!("IDs: {:?}", ids_iter.collect::<Vec<(usize, u32)>>());

    let first_val = ids_iter.next().unwrap();
    let mut prev_time: u64 = first_val.0 as u64;
    let mut prev_lcm: u64 = first_val.1 as u64;

    let second_val = ids_iter.next().unwrap();
    let mut n2: u64 = second_val.1 as u64;
    let mut offset: u64 = second_val.0 as u64;

    let mut count = 2;
    for id in ids_iter {
        if let Some((time, lcm)) = find_time_at_offset(prev_time, prev_lcm, n2, offset) {
            println!("Time for first {} buses : {:?}", count, time);

            prev_time = time;
            prev_lcm = lcm;
            n2 = id.1 as u64;
            offset = id.0 as u64;

            count += 1;
        }
    }

    if let Some((time, _lcm)) = find_time_at_offset(prev_time, prev_lcm, n2, offset) {
        println!("Final Time for list with {} buses : {:?}", count, time);
    }
}

// Will be faster if prev_lcm > n2. prev_lcm is intended to be the lcm of previous values
// and prev_time is the previously found time.
// Set prev_time = 0 if you just want the result for two numbers which would be prev_lcm and n2.
fn find_time_at_offset(prev_time: u64, prev_lcm: u64, n2: u64, offset: u64) -> Option<(u64, u64)> {
    let new_lcm: u64 = num::Integer::lcm(&prev_lcm, &n2);

    for time in (prev_time..=(new_lcm + prev_time)).step_by(prev_lcm as usize) {
        if (time + offset) % n2 == 0 {
            return Some((time, new_lcm));
        }
    }

    None
}
