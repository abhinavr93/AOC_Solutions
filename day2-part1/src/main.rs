use std::str::FromStr;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead,BufReader};

struct PasswordPolicyChecker {

	first_num: u32,
	second_num: u32,
	check_char: char,
	password: String
}

impl FromStr for PasswordPolicyChecker {

	type Err = Box<dyn Error>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {

		let mut iter = s.split_whitespace();

		let nums: Vec<&str> = (iter.next().unwrap()).split('-')
                                 .collect();

        let first_num = nums[0].parse::<u32>()?;
        let second_num = nums[1].parse::<u32>()?;

        let c = (iter.next().unwrap()).trim_matches(|p| p == ':').parse::<char>()?;

        let pass: String = iter.next().unwrap().to_string();

        Ok(PasswordPolicyChecker{
        	first_num,
        	second_num,
        	check_char: c,
        	password: pass
        })
	}
}

impl PasswordPolicyChecker {

	fn check_password(&self) -> bool {

		let char_count: u32 = self.password.as_str().matches(self.check_char).count() as u32;

		char_count >= self.first_num && char_count <= self.second_num
	}
}
fn main() {
    let input = File::open("input.txt").unwrap();
    let line_iter = BufReader::new(input).lines();

    let mut final_count: u32 = 0;
    let mut total_count: u32 = 0;

    for line_item in line_iter {

    	let item = PasswordPolicyChecker::from_str(&line_item.unwrap()).unwrap();

    	if item.check_password() {

    		final_count += 1;

    	}

    	total_count += 1; 

    }

    println!("\n Number of valid passwords : {}, among {} total passwords\n", final_count, total_count);
}
