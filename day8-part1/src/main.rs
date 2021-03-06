use std::fs::File;
use std::io::{BufRead, BufReader};

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

fn execute_instructions(mut instructions: Vec<(bool, String, i32)>) -> i32 {
    let mut acc = 0;

    let mut cur_i = 0;

    while cur_i < instructions.len() && !instructions[cur_i].0 {
        println!(
            "Executing instruction at index {}: {} {}",
            cur_i, instructions[cur_i].1, instructions[cur_i].2
        );
        instructions[cur_i].0 = true;
        if instructions[cur_i].1 == *"acc" {
            acc += instructions[cur_i].2;
            cur_i += 1;
        } else if instructions[cur_i].1 == *"jmp" {
            cur_i = add(cur_i, instructions[cur_i].2);
        } else {
            cur_i += 1;
        }
    }
    if cur_i < instructions.len() && instructions[cur_i].0 {
        println!(
            "Instruction at index {}: {} {} has repeated! Exited.",
            cur_i, instructions[cur_i].1, instructions[cur_i].2
        );
    }
    acc
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let mut instructions: Vec<(bool, String, i32)> = Vec::new();

    while let Some(Ok(line)) = line_iter.next() {
        let parsed_line = line.trim().split(' ').collect::<Vec<&str>>();
        let instruction_string: String = parsed_line[0].trim().to_string();
        let instruction_val: i32 = parsed_line[1].trim().parse::<i32>().unwrap();

        instructions.push((false, instruction_string, instruction_val));
    }

    println!(
        "\nValue of accumulator before getting into an infinite loop: {}\n",
        execute_instructions(instructions)
    );
}
