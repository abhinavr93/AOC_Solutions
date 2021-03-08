use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct State(Vec<Vec<char>>);

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in &self.0 {
            for c in v {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn get_neighbour_indices(row: usize, col: usize) -> Vec<(usize, usize)> {

	let indices: Vec<(usize, usize)> = Vec::new();

	indices

}

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let mut seat_state: Vec<Vec<char>> = Vec::new();

    while let Some(Ok(line)) = line_iter.next() {
        seat_state.push(line.chars().collect::<Vec<char>>());
    }

    println!("{}", State(seat_state));
}
