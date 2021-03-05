use std::collections::HashMap;
use std::collections::HashSet;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn create_bag_rules_map() -> HashMap<String, Vec<(String, u32)>> {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let mut bag_rules: HashMap<String, Vec<(String, u32)>> = HashMap::new();

    while let Some(Ok(line)) = line_iter.next() {
        let mut part_iter = line.split("bags contain");

        let color_key: String = part_iter.next().unwrap().trim().to_string();

        let inner_part_iter = part_iter.next().unwrap().split(',');

        let mut bag_vec: Vec<(String, u32)> = Vec::new();

        for inner_part in inner_part_iter {
            let v: Vec<&str> = inner_part.trim().split(' ').collect();

            if v[0] != "no" {
                let num: u32 = v[0].parse::<u32>().unwrap();
                let bag_string: String = [v[1], v[2]].join(" ").to_string();

                bag_vec.push((bag_string, num));
            }
        }

        bag_rules.insert(color_key, bag_vec);
    }

    bag_rules
}

fn get_bags_containing(
    bag_color: &str,
    bag_map: &HashMap<String, Vec<(String, u32)>>,
) -> Option<HashSet<String>> {
    let mut return_set: HashSet<String> = HashSet::new();
    for (key, value) in bag_map.iter() {
        for (color, _num) in value {
            if color.as_str() == bag_color {
                return_set.insert(key.to_string());
            }
        }
    }

    if return_set.is_empty() {
        //Break recursion
        return None;
    } else {
        let current_fixed_set = return_set.clone();

        for s in current_fixed_set.iter() {
            if let Some(next_level_set) = get_bags_containing(&s, &bag_map) {
                return_set.extend(next_level_set);
            }
        }
    }

    Some(return_set)
}

fn main() {
    let bag_map = create_bag_rules_map();
    let check_color: &str = "shiny gold";

    if let Some(container_bags) = get_bags_containing(check_color, &bag_map) {
        println!(
            "\nNumber of bags containing atleast 1 {} bag : {} which are, ",
            check_color,
            container_bags.len()
        );

        for (i, bag_color) in container_bags.iter().enumerate() {
            println!("{}: {}", i + 1, bag_color);
        }
    } else {
        println!(
            "\nNumber of bags containing atleast 1 {} bag : 0",
            check_color
        );
    }
}
