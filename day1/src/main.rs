use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    
    let input = File::open("input.txt").unwrap();
    let mut items : Vec<u32> = BufReader::new(input).lines().map(|line| line.unwrap().parse::<u32>().unwrap()).collect();

    items.sort_unstable();

    for a in &items {
        for b in &items {
            for c in &items {
                if a + b + c == 2020 {
                    println!("{}\n", a * b * c);
                    return;
                }
            }
        }
    }
}
