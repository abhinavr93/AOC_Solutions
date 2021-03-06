use std::fs::File;
use std::io::{BufRead, BufReader};

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

fn change_instruction(instr: &mut (String, i32)) {
    if instr.0 == *"jmp" {
        instr.0 = "nop".to_string();
    } else if instr.0 == *"nop" {
        instr.0 = "jmp".to_string();
    }
}

fn execute_one(cur_i: usize, acc: i32, instructions: &[(String, i32)]) -> (usize, i32) {
    if instructions[cur_i].0 == *"acc" {
        (cur_i + 1, acc + instructions[cur_i].1)
    } else if instructions[cur_i].0 == *"jmp" {
        (add(cur_i, instructions[cur_i].1), acc)
    } else {
        (cur_i + 1, acc)
    }
}

fn execute_instructions(mut instructions: Vec<(String, i32)>) -> i32 {
    let mut i_stack: Vec<usize> = Vec::new();
    let mut acc_stack: Vec<i32> = Vec::new();
    let mut freeze_max_stack_len: bool = false;
    let mut acc = 0;
    let mut count = 0;
    let mut prev_try_i = 0;
    let mut cur_i = 0;
    while cur_i < instructions.len() && count < instructions.len() {
        // Check if instruction repeats
        if i_stack.contains(&cur_i) {
            // Reset the current index, accumulator and executed count
            cur_i = i_stack.pop().unwrap();
            acc = acc_stack.pop().unwrap();
            count = i_stack.len();
            change_instruction(&mut instructions[cur_i]);
            if !freeze_max_stack_len {
                // Freeze the maximum stack length when the problem area is encountered.
                freeze_max_stack_len = true;
            } else {
                change_instruction(&mut instructions[prev_try_i]);
            }
            prev_try_i = cur_i;
        } else {
            //Execute
            let result = execute_one(cur_i, acc, &instructions);
            count += 1;
            if !freeze_max_stack_len {
                i_stack.push(cur_i);
                acc_stack.push(acc);
            }
            //Update current index and accumulator
            cur_i = result.0;
            acc = result.1;
        }
    }

    if !(0..instructions.len()).contains(&cur_i) || count >= instructions.len() {
        println!("Instructions' execution success.");
    }
    acc
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let mut instructions: Vec<(String, i32)> = Vec::new();

    while let Some(Ok(line)) = line_iter.next() {
        let parsed_line = line.trim().split(' ').collect::<Vec<&str>>();
        let instruction_string: String = parsed_line[0].trim().to_string();
        let instruction_val: i32 = parsed_line[1].trim().parse::<i32>().unwrap();

        instructions.push((instruction_string, instruction_val));
    }

    println!(
        "\nValue of accumulator after termination: {}",
        execute_instructions(instructions)
    );
}
