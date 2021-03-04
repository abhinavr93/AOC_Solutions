use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_keys(s: &str) -> Vec<String> {
    let mut key_vec: Vec<String> = Vec::new();
    for entry in s.split_whitespace() {
        key_vec.push(entry.split(':').next().unwrap().to_string());
    }

    key_vec
}

fn check_validity_from_keys(key_vec: &[String]) -> bool {
    key_vec.len() == 8 || (key_vec.len() == 7 && key_vec.iter().find(|&x| x == "cid") == None)
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let line_iter = BufReader::new(input).lines();

    let mut passport_num = 0;
    let mut valid_passport_num = 0;

    let mut passport_lines: Vec<String> = Vec::new();
    for line in line_iter {
        // Check if line is blank
        let s = line.unwrap();
        if s.trim() == "" {
            let keys: Vec<String> = parse_keys(&passport_lines.join(" "));
            if check_validity_from_keys(&keys) {
                valid_passport_num += 1;
            }
            passport_num += 1;
            passport_lines.clear();
        } else {
            passport_lines.push(s);
        }
    }

    let keys: Vec<String> = parse_keys(&passport_lines.join(" "));
    if check_validity_from_keys(&keys) {
        valid_passport_num += 1;
    }
    passport_num += 1;
    passport_lines.clear();

    println!(
        "\n Number of valid passports is {} among a total of {} passports. \n",
        valid_passport_num, passport_num
    );
}
