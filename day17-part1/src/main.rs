use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    const N: usize = 6;
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let mut init_state: HashMap<i32, Vec<Vec<char>>> = HashMap::new();

    let mut init_slice: Vec<Vec<char>> = Vec::new();

    while let Some(Ok(line)) = line_iter.next() {
        init_slice.push(line.chars().collect::<Vec<char>>());
    }

    init_state.insert(0, init_slice);

    //println!("\nInitial State:\n{}", State(&init_state));

    let mut cur_state = init_state.clone();
    for _i in 0..N {
        cur_state = get_next_state(&mut cur_state);
    }

    println!("\nAfter {} cycles:", N);
    //println!("Old State:\n{}", State(&init_state));
    //println!("New State:\n{}", State(&cur_state));
    println!(
        "\nActive count in new state: {}\n",
        count_all_active(&cur_state)
    );
}

fn get_next_state(old_state: &mut HashMap<i32, Vec<Vec<char>>>) -> HashMap<i32, Vec<Vec<char>>> {
    mutate_old_state(old_state);
    let mut new_state = old_state.clone();

    for z in old_state.keys() {
        let mut_slice_ref = new_state.entry(*z).or_insert(Vec::new());
        *mut_slice_ref = get_new_slice(*z, &old_state);
    }

    new_state
}

fn get_new_slice(z: i32, mutated_old_state: &HashMap<i32, Vec<Vec<char>>>) -> Vec<Vec<char>> {
    let mut new_slice: Vec<Vec<char>> = mutated_old_state[&z].clone();
    let state = State(&mutated_old_state);
    for i in 0..mutated_old_state[&z].len() {
        for j in 0..mutated_old_state[&z][i].len() {
            let count = count_active_neighbours((i, j, z), &mutated_old_state);
            if let Some(c) = state.get(i, j, z) {
                if c == '#' && (count != 2 && count != 3) {
                    new_slice[i][j] = '.';
                } else if c == '.' && count == 3 {
                    new_slice[i][j] = '#';
                }
            }
        }
    }

    new_slice
}

fn mutate_old_state(old_state: &mut HashMap<i32, Vec<Vec<char>>>) {
    let z_min: i32 = *old_state.keys().min().unwrap();
    let z_max: i32 = *old_state.keys().max().unwrap();

    let orig_rowlen: usize = old_state[&z_min].len();
    let orig_collen: usize = old_state[&z_min][0].len();

    for z in z_min..=z_max {
        let mut_slice_ref = old_state.entry(z).or_insert(Vec::new());
        for i in 0..orig_rowlen {
            mut_slice_ref[i].insert(0, '.');
            mut_slice_ref[i].push('.');
        }
        mut_slice_ref.insert(0, vec!['.'; orig_collen + 2]);
        mut_slice_ref.push(vec!['.'; orig_collen + 2]);
    }

    old_state.insert(z_min - 1, vec![vec!['.'; orig_collen + 2]; orig_rowlen + 2]);
    old_state.insert(z_max + 1, vec![vec!['.'; orig_collen + 2]; orig_rowlen + 2]);
}

fn count_active_neighbours(
    p: (usize, usize, i32),
    old_state: &HashMap<i32, Vec<Vec<char>>>,
) -> usize {
    let orig_state = State(old_state);
    let mut active_count = 0;
    let mut start_row = p.0;
    let mut start_col = p.1;
    if p.0 > 0 {
        start_row = p.0 - 1;
    }
    if p.1 > 0 {
        start_col = p.1 - 1;
    }

    for z in p.2 - 1..=p.2 + 1 {
        for i in start_row..=p.0 + 1 {
            for j in start_col..=p.1 + 1 {
                if let Some(char_val) = orig_state.get(i, j, z) {
                    if char_val == '#' && !(i == p.0 && j == p.1 && z == p.2) {
                        active_count += 1;
                    }
                }
            }
        }
    }
    active_count
}

fn count_all_active(state: &HashMap<i32, Vec<Vec<char>>>) -> usize {
    let mut count = 0;
    let st = State(&state);
    for &z in state.keys() {
        for i in 0..state[&z].len() {
            for j in 0..state[&z][i].len() {
                if let Some(char_val) = st.get(i, j, z) {
                    if char_val == '#' {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[derive(PartialEq)]
struct State<'a>(&'a HashMap<i32, Vec<Vec<char>>>);

impl fmt::Display for State<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut keys = self.0.keys().collect::<Vec<&i32>>();
        keys.sort_unstable();
        for z in keys {
            write!(f, "At z = {}\n", z)?;
            for v in &self.0[z] {
                for c in v {
                    write!(f, "{}", c)?;
                }
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl State<'_> {
    fn get(&self, row: usize, col: usize, z: i32) -> Option<char> {
        if let Some(slice) = self.0.get(&z) {
            if let Some(row_vec) = slice.get(row) {
                if let Some(char_val) = row_vec.get(col) {
                    return Some(*char_val);
                }
            }
        }
        None
    }
}
