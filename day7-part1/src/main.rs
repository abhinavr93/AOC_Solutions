use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn create_bag_rules_map() -> HashMap<String, Vec<(String, u32)>> {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let mut bag_rules: HashMap<String, Vec<(String, u32)>> = HashMap::new();

    while let Some(Ok(line)) = line_iter.next() {
        let mut part_iter = line.split("bags contain");

        let color_key: String = part_iter.next().unwrap().trim().to_string();

        println!("Color Key: {}", color_key);

        let mut inner_part_iter = part_iter.next().unwrap().split(',');

        let mut bag_vec: Vec<(String, u32)> = Vec::new();

        while let Some(inner_part) = inner_part_iter.next() {
            let v: Vec<&str> = inner_part.trim().split(' ').collect();

            println!("\t Split inner parts: {}, {}, {}", v[0], v[1], v[2]);

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
) -> Option<Vec<String>> {
    let mut return_vec: Vec<String> = Vec::new();
    for (key, value) in bag_map.into_iter() {
        for (color, _num) in value {
            if color.as_str() == bag_color {
                return_vec.push(key.to_string());
            }
        }
    }

    if return_vec.len() == 0 {
        return None;
    }

    Some(return_vec)
}

fn main() {
    let bag_map = create_bag_rules_map();

    let mut first_level_bags = get_bags_containing("shiny gold", &bag_map).unwrap();

    for s in first_level_bags {
        println!("Shiny Gold Bag contained by: {} bag", s);
    }

    if let None = get_bags_containing("dark orange", &bag_map) {
        println!("Dark Orange Bag contained by no bag");
    }
}
