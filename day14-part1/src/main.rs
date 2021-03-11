use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let mut mem: HashMap<u32, [bool; 36]> = HashMap::new();
    let mut mask: HashMap<usize, bool> = HashMap::new();

    while let Some(Ok(line)) = line_iter.next() {
        let mut split_strs = line.split(" = ");
        let first_part = split_strs.next().unwrap().trim();
        if first_part == "mask" {
            mask = split_strs
                .next()
                .unwrap()
                .trim()
                .chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    if c != 'X' {
                        Some((i, c.to_digit(10).unwrap() != 0))
                    } else {
                        None
                    }
                })
                .collect();

            println!("Mask: {:?}", mask);
        } else {
            let mem_key: u32 = first_part
                .trim_matches(|c: char| !c.is_numeric())
                .parse()
                .unwrap();

            let val: u64 = split_strs.next().unwrap().trim().parse().unwrap();
            mem.insert(mem_key, apply_mask(val, &mask));

            println!(
                "Memory Key: {}, Unmasked value: {}, Masked value: {}",
                mem_key,
                val,
                get_decimal(mem[&mem_key])
            );
        }
    }

    //println!("\nFinal Memory Map: {:?}", mem);
    println!("\nFinal Sum: {}", get_final_sum(&mem));
}

fn apply_mask(val: u64, mask: &HashMap<usize, bool>) -> [bool; 36] {
    let mut binary_vec: Vec<bool> = format!("{:b}", val)
        .chars()
        .map(|c| c.to_digit(2).unwrap() != 0)
        .collect();

    let mut arr = [false; 36];
    let mut n = 35;

    while let Some(val) = binary_vec.pop() {
        arr[n] = val;
        n -= 1;
    }

    for (&key, &val) in mask.iter() {
        arr[key] = val;
    }

    arr
}

fn get_decimal(arr: [bool; 36]) -> u64 {
    let mut val: u64 = 0;
    for i in (0..36).rev() {
        if arr[i] {
            val += 2u64.pow(35 - i as u32);
        }
    }
    val
}

fn get_final_sum(mem: &HashMap<u32, [bool; 36]>) -> u64 {
    mem.values().map(|&arr| get_decimal(arr)).sum()
}
