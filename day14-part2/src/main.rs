use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, HashSet};

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let mut mem: HashMap<u64, u64> = HashMap::new();
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
                .filter_map(|(i, c)| match c {
                    '1' => Some((i, true)),
                    'X' => Some((i, false)),
                    _ => None,
                })
                .collect();

            println!("Mask: {:?}", mask);
        } else {
            let mem_key: u64 = first_part
                .trim_matches(|c: char| !c.is_numeric())
                .parse()
                .unwrap();

            let val: u64 = split_strs.next().unwrap().trim().parse().unwrap();
            let new_mem_keys: HashSet<u64> = apply_mask(mem_key, &mask);
            for &key in new_mem_keys.iter() {
                mem.insert(key, val);
            }

            println!(
                "Original Memory Key: {}, New Memory Keys : {:?}, Value: {}\n--------------",
                mem_key, new_mem_keys, val
            );
        }
    }

    //println!("\nFinal Memory Map: {:?}", mem);
    println!("\nFinal Sum: {}\n", get_final_sum(&mem));
}

fn apply_mask(val: u64, mask: &HashMap<usize, bool>) -> HashSet<u64> {
    let mut arr = get_binary_array(val);

    for (&key, &val) in mask.iter().filter(|(&_k, &v)| v) {
        arr[key] = val;
    }

    find_keys(arr, &mask.iter().filter(|(&_k, &v)| !v).collect())
}

fn find_keys(arr: [bool; 36], mask: &HashMap<&usize, &bool>) -> HashSet<u64> {
    let mut mem_keys: HashSet<u64> = HashSet::new();
    let mut binary_mem = arr;
    let mut submask = mask.clone();
    if mask.len() == 1 {
        // break recursion
        let &&key = mask.keys().next().unwrap();
        binary_mem[key] = true;
        mem_keys.insert(get_decimal(binary_mem));
        binary_mem[key] = false;
        mem_keys.insert(get_decimal(binary_mem));
    } else {
        for (&&key, _val) in mask.iter() {
            submask.remove(&key);
            binary_mem[key] = true;
            mem_keys.extend(find_keys(binary_mem, &submask));
            binary_mem[key] = false;
            mem_keys.extend(find_keys(binary_mem, &submask));
        }
    }

    mem_keys
}

fn get_binary_array(dec_val: u64) -> [bool; 36] {
    let mut binary_vec: Vec<bool> = format!("{:b}", dec_val)
        .chars()
        .map(|c| c.to_digit(2).unwrap() != 0)
        .collect();

    let mut arr = [false; 36];
    let mut n = 35;

    while let Some(val) = binary_vec.pop() {
        arr[n] = val;
        n -= 1;
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

fn get_final_sum(mem: &HashMap<u64, u64>) -> u64 {
    mem.values().sum()
}
