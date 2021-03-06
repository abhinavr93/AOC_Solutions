use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut num_vec: Vec<u32> = BufReader::new(input)
        .lines()
        .map(|p| p.unwrap().parse::<u32>().unwrap())
        .collect();

    num_vec.sort_unstable();
    num_vec.insert(0, 0);
    num_vec.push(*num_vec.last().unwrap() + 3);

    println!("{:?}", num_vec);
    let diff_vec: Vec<u32> = num_vec
        .iter()
        .zip(num_vec.iter().skip(1))
        .map(|(first, second)| second - first)
        .collect();

    let product =
        diff_vec.iter().filter(|&&x| x == 1).count() * diff_vec.iter().filter(|&&x| x == 3).count();

    println!("\nDiff Vec:\n {:?}", diff_vec);

    println!("\n 1-diff count * 3-diff count: {}", product);

    let mut three_diff_indices: Vec<usize> = diff_vec
        .iter()
        .enumerate()
        .filter(|(_i, &x)| x == 3)
        .map(|(i, &_x)| i)
        .collect();

    three_diff_indices.insert(0, 0);

    let index_diff_vec: Vec<usize> = three_diff_indices
        .iter()
        .zip(three_diff_indices.iter().skip(1))
        .filter(|(&first, &second)| second > first + 2usize)
        .map(|(first, second)| match first {
            0 => second - first - 1,
            _ => second - first - 2,
        })
        .collect();

    println!("\n 3-diff indices vector : {:?}", three_diff_indices);

    println!("\n 3-diff indices diff vector : {:?}", index_diff_vec);

    let total_product: u64 = index_diff_vec
        .iter()
        .map(|&x| match x {
            3 => 7_u64,
            2 => 4_u64,
            1 => 2_u64,
            _ => 0_u64,
        })
        .product();

    println!("\n Total arrangments possible: {}", total_product);
}
