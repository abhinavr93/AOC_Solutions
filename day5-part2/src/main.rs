use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();
    let mut max_num = 0;
    let mut min_num = 1024;
    let mut seat_nums: Vec<u32> = Vec::new();
    while let Some(Ok(line)) = line_iter.next() {
        let bdigits: Vec<u32> = line
            .chars()
            .map(|p| match p {
                'F' => 0,
                'B' => 1,
                'L' => 0,
                'R' => 1,
                _ => 0,
            })
            .collect();

        let mut sum = 0;
        for (i, digit) in bdigits.iter().enumerate() {
            sum += digit * 2u32.pow((bdigits.len() - i - 1) as u32);
        }

        if sum > max_num {
            max_num = sum;
        }

        if sum < min_num {
            min_num = sum;
        }

        seat_nums.push(sum);
    }

    println!("\n Max Seat Number: {}", max_num);
    println!("\n Min Seat Number: {}", min_num);

    seat_nums.sort_unstable();

    for i in 0..(seat_nums.len() - 1) {
        if seat_nums[i] + 2 == seat_nums[i + 1] {
            println!("\n Missing seat number : {} \n", seat_nums[i] + 1);
        }
    }
}
