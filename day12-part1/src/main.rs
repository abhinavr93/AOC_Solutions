use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Orientation {
	pos: (i32, i32),
	dir: (i32, i32)
}

impl Orientation {

	fn manhattan_dist(&self) -> u32 {
		(self.pos.0.abs() + self.pos.1.abs()) as u32
	}

	fn move_dir(&mut self, move_val: i32, angle: i32 ) {
		let mut or = Orientation{pos: self.pos, dir: (1,0)};

		or.rotate(angle);
		or.move_forward(move_val);

		self.pos = or.pos;
		//self.dir = or.dir;
	}

	fn move_forward(&mut self, move_val: i32) {
		self.pos = (self.pos.0 + move_val*self.dir.0, self.pos.1 + move_val*self.dir.1);
	}

	fn rotate(&mut self, angle: i32) {

		let rad_angle = std::f64::consts::PI * (angle as f64)/180.0;
		let cos = rad_angle.cos();
		let sin = rad_angle.sin();
		let x_dir: f64 = self.dir.0 as f64;
		let y_dir: f64 = self.dir.1 as f64;
		
		self.dir = ((x_dir * cos - y_dir * sin).round() as i32, (x_dir * sin + y_dir * cos).round() as i32);
	}

	fn update(&mut self, c: char, val: i32) {

		match c {
			'E' => self.move_dir(val, 0),
			'N' => self.move_dir(val, 90),			
			'W' => self.move_dir(val, 180),
			'S' => self.move_dir(val, 270),
			'F' => self.move_forward(val),
			'R' => self.rotate(-1 * val),
			'L' => self.rotate(val),
			 _ => {}
		}
	}
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let mut state = Orientation{pos: (0,0), dir: (1,0)};
    println!("Initial -> {:?}", state);

     while let Some(Ok(line)) = line_iter.next() {
        let c: char = line.chars().next().unwrap();
        let val: i32 = line.trim_matches(char::is_alphabetic).parse::<i32>().unwrap();

        state.update(c, val);
        println!("After {}{} -> {:?}", c, val, state);

    }

    println!("Final Manhattan Distance: {}", state.manhattan_dist());
}
