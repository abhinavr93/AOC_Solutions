use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, HashSet};

use regex::Regex;

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let rules = read_rules(&mut line_iter);

    let my_ticket_vals = get_my_ticket_vals(&mut line_iter);

    let mut ticket_line_iter = line_iter
        .skip_while(|line| line.as_ref().unwrap().trim() != "nearby tickets:")
        .skip(1);

    let mut valid_tickets: Vec<Vec<usize>> = Vec::new();
    while let Some(Ok(line)) = ticket_line_iter.next() {
        let vals: Vec<usize> = line
            .split(',')
            .map(|p| p.parse::<usize>().unwrap())
            .collect();
        let invalid_vals: Vec<&usize> = vals
            .iter()
            .filter(|&val| {
                !rules.iter().any(|r| {
                    (val >= &r.1[0] && val <= &r.1[1]) || (val >= &r.1[2] && val <= &r.1[3])
                })
            })
            .collect();

        if invalid_vals.len() == 0 {
            valid_tickets.push(vals);
        }
    }

    let fields = find_fields(&valid_tickets, &rules);

    let final_product: usize = fields
        .iter()
        .filter_map(|(i, s)| {
            if s.split_whitespace().next().unwrap().trim() == "departure" {
                Some(my_ticket_vals[*i])
            } else {
                None
            }
        })
        .product();

    println!(
        "\n\nFinal Product of departure fields in my ticket: {}",
        final_product
    );
}

fn find_fields(
    tickets: &Vec<Vec<usize>>,
    rules: &HashMap<String, Vec<usize>>,
) -> Vec<(usize, String)> {
    let mut fields: Vec<(usize, String)> = Vec::new();
    let mut remaining_rules = rules.clone();
    let mut remaining_cols: HashSet<usize> = (0..rules.len()).collect();

    while !remaining_rules.is_empty() {
        if let Some(field) = find_field(&tickets, &remaining_rules, &remaining_cols) {
            remaining_cols.remove(&field.0);
            remaining_rules.remove(&field.1);
            fields.push(field);
        }
    }
    fields
}

fn find_field(
    tickets: &Vec<Vec<usize>>,
    rules: &HashMap<String, Vec<usize>>,
    cols: &HashSet<usize>,
) -> Option<(usize, String)> {
    for &col in cols {
        let valid_rules = rules
            .iter()
            .filter(|(_key, r)| {
                (0..tickets.len()).all(|row| {
                    let val = tickets[row][col];
                    (val >= r[0] && val <= r[1]) || (val >= r[2] && val <= r[3])
                })
            })
            .collect::<Vec<(&String, &Vec<usize>)>>();

        if valid_rules.len() == 1 {
            return Some((col, valid_rules[0].0.to_string()));
        }
    }
    None
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
