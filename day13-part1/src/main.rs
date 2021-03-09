use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let time: u32 = line_iter.next().unwrap().unwrap().parse::<u32>().unwrap();

    let ids: Vec<u32> = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let waiting_times: Vec<u32> = ids.iter().map(|x| (x * (time / x) + x) - time).collect();

    println!("IDs: {:?}\n Waiting Times: {:?}", ids, waiting_times);

    let min_waiting_time = waiting_times
        .iter()
        .enumerate()
        .min_by(|(_i1, x1), (_i2, x2)| x1.cmp(x2))
        .unwrap();

    println!("Min Waiting Time: {:?}", min_waiting_time.1);
    println!(
        "Required Answer: {:?}",
        min_waiting_time.1 * ids[min_waiting_time.0]
    );
}
