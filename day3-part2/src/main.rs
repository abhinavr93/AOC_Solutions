use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_xpos(i: usize, mx: usize, len: usize) -> usize {
    ((i + 1) * mx) % len
}

fn main() {
    let slopes: Vec<(usize, usize)> = vec![(3, 1), (1, 1), (5, 1), (7, 1), (1, 2)];

    let mut final_product = 1;

    for slope in slopes {
        let mx: usize = slope.0;
        let my: usize = slope.1;

        let input = File::open("input.txt").unwrap();
        let mut line_iter = BufReader::new(input).lines().step_by(my);
        let line_len = line_iter.next().unwrap().unwrap().len();

        let mut tree_count = 0;
        let mut line_count = 0;

        for (i, line) in line_iter.enumerate() {
            let s = line.unwrap();
            let mut char_iter = s.chars();
            if char_iter.nth(get_xpos(i, mx, line_len)).unwrap() == '#' {
                tree_count += 1;
            }

            line_count += my;
        }

        println!("\n For (x-slope: {}, y-slope: {})", mx, my);
        println!("\n\t The number of trees('#') found are : {} ", tree_count);
        println!("\n\t Total number of lines : {}", line_count);

        final_product *= tree_count;
    }

    println!("\n\n FINAL PRODUCT : {}", final_product);
}
