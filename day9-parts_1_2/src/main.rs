use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn create_sums_set(n_vec: Vec<u64>) -> HashSet<u64> {
    let mut sums: HashSet<u64> = HashSet::new();

    for i in 0..n_vec.len() {
        for j in i + 1..n_vec.len() {
            if n_vec[i] != n_vec[j] {
                sums.insert(n_vec[i] + n_vec[j]);
            }
        }
    }
    sums
}

fn main() {
    const N: usize = 25;

    let input = File::open("input.txt").unwrap();
    let num_vec: Vec<u64> = BufReader::new(input)
        .lines()
        .map(|p| p.unwrap().parse::<u64>().unwrap())
        .collect();

    let mut i = 0;
    loop {
        let sums_set = create_sums_set(num_vec[i..i + N].to_vec());

        if !sums_set.contains(&num_vec[i + N]) {
            break;
        } else {
            i += 1;
        }
    }

    let invalid_idx: usize = i + N;
    let invalid_num: u64 = num_vec[i + N];
    println!(
        "First Inavlid number is at index {} which is: {}",
        invalid_idx, invalid_num
    );

    for slice_size in 0..invalid_idx {
        for start_i in (0..invalid_idx - slice_size).rev() {
            let check_slice = num_vec[start_i..start_i + slice_size].to_vec();
            if check_slice.iter().sum::<u64>() == invalid_num {
                let max = check_slice.iter().max().unwrap();
                let min = check_slice.iter().min().unwrap();
                println!(
                    "\nFor slice {:?} : \n\t Sum: {},\n\t Max: {},\n\t Min: {},\n\t Max+Min: {}",
                    check_slice,
                    invalid_num,
                    max,
                    min,
                    max + min
                );
                return;
            }
        }
    }
}
