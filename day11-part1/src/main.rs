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

fn get_new_seat(row: usize, col: usize, orig_seats: &Vec<Vec<char>>) -> Option<char> {
    let orig_state = State(orig_seats);
    let mut occupied_count = 0;
    let mut start_row = row;
    let mut start_col = col;
    if row > 0 {
        start_row = row - 1;
    }
    if col > 0 {
        start_col = col - 1;
    }
    for i in start_row..=row + 1 {
        for j in start_col..=col + 1 {
            if let Some(char_val) = orig_state.get(i, j) {
                if char_val == '#' && !(i == row && j == col) {
                    occupied_count += 1;
                }
            }
        }
    }

    if let Some(char_val) = orig_state.get(row, col) {
        if char_val == '#' && occupied_count >= 4 {
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
            "After round {}, occupied seats: {}\n{}",
            iter,
            count_occupied(&seat_state),
            State(&seat_state)
        );
    }
}
