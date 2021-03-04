use std::fs::File;
use std::io::{BufRead, BufReader};

fn validate_yr(value: &str, min: u32, max: u32) -> bool {
    if let Ok(yr) = value.parse::<u32>() {
        yr >= min && yr <= max
    } else {
        false
    }
}

fn validate_hgt(value: &str) -> bool {
    let unit: String = value.chars().filter(|p| p.is_alphabetic()).collect();

    if let Ok(ht) = value.split(&unit).next().unwrap().parse::<i32>() {
        if unit == *"cm" {
            (150..=193).contains(&ht)
        } else if unit == *"in" {
            (59..=76).contains(&ht)
        } else {
            false
        }
    } else {
        false
    }
}

fn validate_hcl(value: &str) -> bool {
    let mut char_iter = value.chars();

    match char_iter.next() {
        Some(c) => c == '#' && char_iter.filter(|p| p.is_digit(16)).count() == 6,
        None => false,
    }
}

fn validate_ecl(value: &str) -> bool {
    let str_vec: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    str_vec.iter().find(|&&x| x == value) != None
}

fn validate_pid(value: &str) -> bool {
    value.chars().filter(|p| p.is_digit(10)).count() == 9
}

fn validate_value(key: &str, value: &str) -> bool {
    match key {
        "byr" => validate_yr(value, 1920, 2002),
        "iyr" => validate_yr(value, 2010, 2020),
        "eyr" => validate_yr(value, 2020, 2030),
        "hgt" => validate_hgt(value),
        "hcl" => validate_hcl(value),
        "ecl" => validate_ecl(value),
        "pid" => validate_pid(value),
        "cid" => true,
        _ => false,
    }
}

fn parse_keys(s: &str) -> Option<Vec<String>> {
    let mut key_vec: Vec<String> = Vec::new();
    for entry in s.split_whitespace() {
        let v: Vec<&str> = entry.split(':').collect();
        let key = v[0].to_string();
        let value = v[1].to_string();

        if !validate_value(&key, &value) {
            return None;
        }
        key_vec.push(key);
    }

    Some(key_vec)
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
            if let Some(keys) = parse_keys(&passport_lines.join(" ")) {
                if check_validity_from_keys(&keys) {
                    valid_passport_num += 1;
                }
            }
            passport_num += 1;
            passport_lines.clear();
        } else {
            passport_lines.push(s);
        }
    }

    if let Some(keys) = parse_keys(&passport_lines.join(" ")) {
        if check_validity_from_keys(&keys) {
            valid_passport_num += 1;
        }
    }
    passport_num += 1;
    passport_lines.clear();

    println!(
        "\n Number of valid passports is {} among a total of {} passports. \n",
        valid_passport_num, passport_num
    );
}
