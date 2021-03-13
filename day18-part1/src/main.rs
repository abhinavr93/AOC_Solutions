use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let mut final_values: Vec<u64> = Vec::new();
    while let Some(Ok(line)) = line_iter.next() {
        let chars = line.chars().filter(|&c| c != ' ').collect::<Vec<char>>();
        if let Some(val) = evaluate(&chars) {
            final_values.push(val);
        }
    }

    println!(
        "Sum of evaluated expressions: {}",
        final_values.iter().sum::<u64>()
    );
}

fn evaluate(chars: &[char]) -> Option<u64> {
    if let None = inner_chars(chars) {
        // Evaluate normally and break recursion
        evaluate_normal(chars)
    } else {
        let mut new_expr = chars.to_vec();
        // Do a recursive iteration until new_expr is a normal expression
        // without parentheses.
        while let Some(inner_expr) = inner_chars(&new_expr.clone()) {
            let inner_val = evaluate(inner_expr.0)?.to_string();
            let insert_vec = inner_val.chars().into_iter();
            new_expr.splice(inner_expr.1..=inner_expr.2, insert_vec);
        }

        evaluate_normal(&new_expr)
    }
}

// Evaluate expression with no parentheses
fn evaluate_normal(chars: &[char]) -> Option<u64> {
    let num_pos = get_num(0, chars)?;
    let mut val = num_pos.0;
    let mut i = num_pos.1 + 1;

    while i < chars.len() {
        let new = get_num(i, chars)?;
        let new_val = new.0;
        match chars[i - 1] {
            '+' => val += new_val,
            '*' => val *= new_val,
            _ => return None,
        }
        i = new.1 + 1;
    }
    Some(val)
}

// Combine consecutive digits to get a number and the position after it ends
fn get_num(i_start: usize, chars: &[char]) -> Option<(u64, usize)> {
    let mut val = chars[i_start].to_digit(10)? as u64;
    let mut j = i_start + 1;
    while let Some(c) = chars.get(j) {
        if let Some(d) = c.to_digit(10) {
            val = val * 10 + d as u64;
            j += 1;
        } else {
            break;
        }
    }
    Some((val, j))
}

// Find the first expression in a parentheses pair and return it along with the position
// of the parentheses
fn inner_chars(chars: &[char]) -> Option<(&[char], usize, usize)> {
    let par_chars = chars
        .iter()
        .enumerate()
        .filter(|(_i, &x)| x == '(' || x == ')')
        .collect::<Vec<(usize, &char)>>();
    let mut iter = par_chars
        .windows(2)
        .filter(|&w| *w[0].1 == '(' && *w[1].1 == ')');
    let pair = iter.next()?;
    let i1 = pair[0].0;
    let i2 = pair[1].0;
    Some((&chars[i1 + 1..i2], i1, i2))
}
