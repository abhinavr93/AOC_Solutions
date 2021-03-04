use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_xpos(i: usize, mx: usize, len: usize) -> usize {
    ((i + 1) * mx) % len
}

fn main() {
    const MX: usize = 3;
    const MY: usize = 1;

    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines().step_by(MY);
    let line_len = line_iter.next().unwrap().unwrap().len();

    let mut tree_count = 0;
    let mut line_count = 0;

    for (i, line) in line_iter.enumerate() {
        let s = line.unwrap();
        println!("{}", s);
        let mut char_iter = s.chars();
        if char_iter.nth(get_xpos(i, MX, line_len)).unwrap() == '#' {
            tree_count += 1;
        }

        line_count += MY;
    }

    println!("\n The number of trees('#') found are : {} ", tree_count);
    println!("\n Total number of lines : {}", line_count);
}
