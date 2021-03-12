use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let rules = read_rules(&mut line_iter);

    let my_ticket_vals = get_my_ticket_vals(&mut line_iter);

    let ticket_line_iter = line_iter
        .skip_while(|line| line.as_ref().unwrap().trim() != "nearby tickets:")
        .skip(1);

    let val_string = ticket_line_iter
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join(",");
    let values = val_string
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let final_sum: usize = values
        .iter()
        .filter(|&val| {
            !rules
                .iter()
                .any(|r| (val >= &r.1[0] && val <= &r.1[1]) || (val >= &r.1[2] && val <= &r.1[3]))
        })
        .sum();

    println!("Rules: {:?}", rules);
    println!("My ticket Values : {:?}", my_ticket_vals);
    println!("All nearby ticket values: {:?}", values);
    println!("Sum of invalid values: {}", final_sum);
}

fn read_rules<B: BufRead>(line_iter: &mut std::io::Lines<B>) -> HashMap<String, Vec<usize>> {
    let re = Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let mut rules: HashMap<String, Vec<usize>> = HashMap::new();

    let rule_iter = line_iter
        .take_while(|line| line.as_ref().unwrap().trim() != "")
        .map(|line| line.unwrap());

    for rule_line in rule_iter {
        let caps = re.captures(&rule_line).unwrap();

        let mut iter = caps.iter().skip(1);
        let key = iter.next().unwrap().map_or("", |m| m.as_str()).to_string();
        let vals: Vec<usize> = iter
            .map(|m| m.unwrap().as_str().parse::<usize>().unwrap())
            .collect();

        rules.insert(key, vals);
    }

    rules
}

fn get_my_ticket_vals<B: BufRead>(line_iter: &mut std::io::Lines<B>) -> Vec<usize> {
    line_iter
        .skip_while(|line| line.as_ref().unwrap().trim() != "your ticket:")
        .nth(1)
        .unwrap()
        .unwrap()
        .split(',')
        .map(|p| p.parse::<usize>().unwrap())
        .collect()
}
