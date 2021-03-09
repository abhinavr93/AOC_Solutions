use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let pos: (i32, i32) = (0,0);

     while let Some(Ok(line)) = line_iter.next() {
        let c: char = line.chars().next().unwrap();
        let val: i32 = line.split(char::is_alphabetic).nth(1).unwrap().parse::<i32>().unwrap();

        println!("{}, {}",c,val);
    }

}
