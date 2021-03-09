use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Orientation {
	pos: (i32, i32),
	ship_pos: (i32, i32)
}

fn rotate(vec: (i32, i32), angle: i32) -> (i32, i32) {
	let rad_angle = std::f64::consts::PI * (angle as f64)/180.0;
		let cos = rad_angle.cos();
		let sin = rad_angle.sin();
		let x_val: f64 = vec.0 as f64;
		let y_val: f64 = vec.1 as f64;

		((x_val * cos - y_val * sin) as i32, (x_val * sin + y_val * cos) as i32)
}

impl Orientation {

	fn manhattan_dist(&self) -> u32 {
		(self.ship_pos.0.abs() + self.ship_pos.1.abs()) as u32
	}

	fn move_dir(&mut self, move_val: i32, angle: i32 ) {
		let dir = rotate((1,0), angle);

		self.pos = (self.pos.0 + move_val*dir.0, self.pos.1 + move_val*dir.1);
	}

	fn move_ship_forward(&mut self, move_times: i32) {
		let x_move = move_times*self.pos.0;
		let y_move = move_times*self.pos.1;
		self.ship_pos = (self.ship_pos.0 + x_move, self.ship_pos.1 + y_move);
		//self.pos = (self.ship_pos.0 + self.pos.0, self.ship_pos.1 + self.pos.1)
	}

	fn rotate(&mut self, angle: i32) {

		self.pos = rotate(self.pos, angle);
	}

	fn update(&mut self, c: char, val: i32) {

		match c {
			'E' => self.move_dir(val, 0),
			'N' => self.move_dir(val, 90),			
			'W' => self.move_dir(val, 180),
			'S' => self.move_dir(val, 270),
			'F' => self.move_ship_forward(val),
			'R' => self.rotate(-1 * val),
			'L' => self.rotate(val),
			 _ => {}
		}
	}
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(input).lines();

    let mut state = Orientation{pos: (10,1), ship_pos: (0,0)};
    println!("Initial -> {:?}", state);

     while let Some(Ok(line)) = line_iter.next() {
        let c: char = line.chars().next().unwrap();
        let val: i32 = line.trim_matches(char::is_alphabetic).parse::<i32>().unwrap();

        state.update(c, val);
        println!("After {}{} -> {:?}", c, val, state);

    }

    println!("Final Manhattan Distance: {}", state.manhattan_dist());
}
