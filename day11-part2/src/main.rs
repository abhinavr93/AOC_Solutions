use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
struct State<'a>(&'a Vec<Vec<char>>);

impl fmt::Display for State<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in self.0 {
            for c in v {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl State<'_> {
    fn get(&self, row: usize, col: usize) -> Option<char> {
        if let Some(row_vec) = self.0.get(row) {
            if let Some(char_val) = row_vec.get(col) {
                return Some(*char_val);
            }
        }
        None
    }
}

fn found_seat(row: usize, col: usize, orig_state: &State) -> (bool, char, u32) {
    if let Some(char_val) = orig_state.get(row, col) {
        match char_val {
            '#' => (true, char_val, 1),
            'L' => (true, char_val, 0),
            _ => (false, char_val, 0),
        }
    } else {
        (true, 'L', 0)
    }
}

fn search_visible_occupied(row: usize, col: usize, orig_seats: &Vec<Vec<char>>) -> u32 {
    let orig_state = State(orig_seats);
    let mut occupied_count = 0;

    let mut flags = [false; 8];
    let mut i_down = row + 1;
    let mut j_forw = col + 1;
    let mut i_up = row;
    let mut j_back = col;

    if row == 0 {
        flags[0] = true;
        flags[1] = true;
        flags[2] = true;
    } else {
        i_up = row - 1;
    }

    if col == 0 {
        flags[0] = true;
        flags[3] = true;
        flags[5] = true;
    } else {
        j_back = col - 1;
    }

    while flags != [true; 8] {
        if !flags[0] {
            let val = found_seat(i_up, j_back, &orig_state);
            flags[0] = val.0;
            occupied_count += val.2;
        }
        if !flags[1] {
            let val = found_seat(i_up, col, &orig_state);
            flags[1] = val.0;
            occupied_count += val.2;
        }
        if !flags[2] {
            let val = found_seat(i_up, j_forw, &orig_state);
            flags[2] = val.0;
            occupied_count += val.2;
        }
        if !flags[3] {
            let val = found_seat(row, j_back, &orig_state);
            flags[3] = val.0;
            occupied_count += val.2;
        }
        if !flags[4] {
            let val = found_seat(row, j_forw, &orig_state);
            flags[4] = val.0;
            occupied_count += val.2;
        }
        if !flags[5] {
            let val = found_seat(i_down, j_back, &orig_state);
            flags[5] = val.0;
            occupied_count += val.2;
        }
        if !flags[6] {
            let val = found_seat(i_down, col, &orig_state);
            flags[6] = val.0;
            occupied_count += val.2;
        }
        if !flags[7] {
            let val = found_seat(i_down, j_forw, &orig_state);
            flags[7] = val.0;
            occupied_count += val.2;
        }

        if i_up == 0 {
            flags[0] = true;
            flags[1] = true;
            flags[2] = true;
        } else {
            i_up -= 1;
        }
        if j_back == 0 {
            flags[0] = true;
            flags[3] = true;
            flags[5] = true;
        } else {
            j_back -= 1;
        }

        i_down += 1;
        j_forw += 1;
    }
    occupied_count
}

fn get_new_seat(row: usize, col: usize, orig_seats: &Vec<Vec<char>>) -> Option<char> {
    let orig_state = State(orig_seats);
    let occupied_count = search_visible_occupied(row, col, orig_seats);

    if let Some(char_val) = orig_state.get(row, col) {
        if char_val == '#' && occupied_count >= 5 {
            return Some('L');
        } else if char_val == 'L' && occupied_count == 0 {
            return Some('#');
        } else {
            return Some(char_val);
        }
    }

    None
}

fn update_state(seats: &mut Vec<Vec<char>>) -> bool {
    let orig_seats = seats.clone();
    for i in 0..seats.len() {
        for j in 0..seats[i].len() {
            seats[i][j] = get_new_seat(i, j, &orig_seats).unwrap();
        }
    }

    if State(seats) == State(&orig_seats) {
        return false;
    }

    true
}

fn count_occupied(seats: &Vec<Vec<char>>) -> u32 {
    let mut count: u32 = 0;
    for i in 0..seats.len() {
        for j in 0..seats[i].len() {
            if seats[i][j] == '#' {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let mut seat_state: Vec<Vec<char>> = Vec::new();

    while let Some(Ok(line)) = line_iter.next() {
        seat_state.push(line.chars().collect::<Vec<char>>());
    }

    println!("\nOriginal State:\n{}", State(&seat_state));

    let mut iter = 0;

    while update_state(&mut seat_state) {
        iter += 1;
        println!(
            "After round {}, occupied seats: {}",
            iter,
            count_occupied(&seat_state)
        );
    }

    println!("\nFinal State:\n{}", State(&seat_state));
}
