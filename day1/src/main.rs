use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    
    let input = File::open("input.txt").unwrap();
    let mut items : Vec<u32> = BufReader::new(input).lines().map(|line| line.unwrap().parse::<u32>().unwrap()).collect();

    items.sort_unstable();

    for (i,a) in items.iter().enumerate() {
        for (j,b) in items.iter().enumerate() {
            for (k,c) in items.iter().enumerate() {
                if a + b + c == 2020 && i != j && i != k && j != k  {
                    println!("\n\n Items that sum to 2020 are : {}, {}, {}", a, b, c);
                    println!("\n Their product is : {}\n", a * b * c);
                    return;
                }
            }
        }
    }
}
