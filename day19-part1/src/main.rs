use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum RuleType {
    Char(char),
    One(usize),
    OneOr(usize, usize),
    Two(usize, usize),
    TwoOr(usize, usize, usize, usize),
    Three(usize, usize, usize),
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let line_iter = BufReader::new(input).lines();

    let rule_map: HashMap<usize, RuleType> = line_iter
        .take_while(|line| line.as_ref().unwrap().trim() != "")
        .filter_map(|line| parse_rule(&line.unwrap()))
        .collect();

    //println!("Rule Map: \n{:?}", rule_map);

    let num = 0;
    let strings = get_strings(num, &rule_map);
    //println!("Strings for {}: {:?}", num, strings);
    //println!("String Length: {}", strings[1].len());
    let len = 24; //strings[0].len();

    let mut iter = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .skip(rule_map.len() + 1)
        .filter(|line| {
            let s = line.as_ref().unwrap().trim();
            s.len() == len
        });

    let mut count = 0;
    while let Some(Ok(s)) = iter.next() {
        if strings.contains(&s) {
            count += 1;
        }
    }

    println!("Valid count: {}", count);
}

fn parse_rule(line: &str) -> Option<(usize, RuleType)> {
    let rule: RuleType;

    let mut parts_iter = line.split(':');
    let part1 = parts_iter.next()?;
    let rule_no = part1.parse::<usize>().ok()?;
    let part2 = parts_iter
        .next()?
        .trim()
        .split(' ')
        .map(|s| s.trim_matches(|c| c == '\"' || c == ' '))
        .collect::<Vec<&str>>();

    if part2.contains(&"a") {
        rule = RuleType::Char('a');
    } else if part2.contains(&"b") {
        rule = RuleType::Char('b');
    } else if part2.contains(&"|") {
        let nums = part2
            .iter()
            .filter_map(|&s| s.parse::<usize>().ok())
            .collect::<Vec<usize>>();
        match nums.len() {
            2 => rule = RuleType::OneOr(nums[0], nums[1]),
            4 => rule = RuleType::TwoOr(nums[0], nums[1], nums[2], nums[3]),
            _ => return None,
        }
    } else {
        let nums = part2
            .iter()
            .filter_map(|&s| s.parse::<usize>().ok())
            .collect::<Vec<usize>>();
        match nums.len() {
            1 => rule = RuleType::One(nums[0]),
            2 => rule = RuleType::Two(nums[0], nums[1]),
            3 => rule = RuleType::Three(nums[0], nums[1], nums[2]),
            _ => return None,
        }
    }

    Some((rule_no, rule))
}

fn get_strings(num: usize, rule_map: &HashMap<usize, RuleType>) -> Vec<String> {
    match rule_map[&num] {
        RuleType::Char(c) => vec![c.to_string()],
        RuleType::One(n) => get_strings(n, &rule_map),
        RuleType::OneOr(n1, n2) => or_strings(
            &mut get_strings(n1, &rule_map),
            &mut get_strings(n2, &rule_map),
        ),
        RuleType::Two(n1, n2) => join_two(get_strings(n1, &rule_map), get_strings(n2, &rule_map)),
        RuleType::TwoOr(n1, n2, n3, n4) => or_strings(
            &mut join_two(get_strings(n1, &rule_map), get_strings(n2, &rule_map)),
            &mut join_two(get_strings(n3, &rule_map), get_strings(n4, &rule_map)),
        ),
        RuleType::Three(n1, n2, n3) => join_three(
            get_strings(n1, &rule_map),
            get_strings(n2, &rule_map),
            get_strings(n3, &rule_map),
        ),
    }
}

fn or_strings(vec1: &mut Vec<String>, vec2: &mut Vec<String>) -> Vec<String> {
    vec1.append(vec2);
    vec1.to_vec()
}

fn join_two(vec1: Vec<String>, vec2: Vec<String>) -> Vec<String> {
    let mut return_vec: Vec<String> = Vec::new();

    for s1 in &vec1 {
        for s2 in &vec2 {
            let joined_string = [s1.to_string(), s2.to_string()].join("");
            return_vec.push(joined_string);
        }
    }

    return_vec
}

fn join_three(vec1: Vec<String>, vec2: Vec<String>, vec3: Vec<String>) -> Vec<String> {
    let mut return_vec: Vec<String> = Vec::new();

    for s1 in &vec1 {
        for s2 in &vec2 {
            for s3 in &vec3 {
                let joined_string = [s1.to_string(), s2.to_string(), s3.to_string()].join("");
                return_vec.push(joined_string);
            }
        }
    }

    return_vec
}
