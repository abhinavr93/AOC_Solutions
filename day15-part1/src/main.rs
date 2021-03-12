use std::collections::HashMap;

fn main() {
    let start_nums: Vec<u32> = vec![6,19,0,5,7,13,1];
    const FINAL_ROUND: usize = 2020;

    // Key is the number. It maps to (prev_index, prev_diff)
    let mut num_map: HashMap<u32, (u32, u32)> = HashMap::new(); 
    for (i,num) in start_nums.iter().enumerate() {

    	num_map.insert(*num, (i as u32, 0));
    } 

    println!("Starting numbers: {:?}", num_map);

    let mut prev_num = *start_nums.last().unwrap();
    for i in start_nums.len()..FINAL_ROUND {
    	let prev_diff = num_map[&prev_num].1;
    	let mut cur_num = 0;
    	if prev_diff == 0 {
    		println!("Previous number: {:?} at index {} is new.", prev_num, num_map[&prev_num].0);
    		
    	} else {
    		println!("Previous number: {} at index {} was repeated before at index {}",
    				prev_num, 
    				num_map[&prev_num].0, 
    				num_map[&prev_num].0 - prev_diff );

    		cur_num = prev_diff;
    	}

    	if num_map.contains_key(&cur_num) {
    		num_map.insert(cur_num, (i as u32, i as u32 - num_map[&cur_num].0))
    	} else {
    		num_map.insert(cur_num, (i as u32,0))
    	};

    	prev_num = cur_num;
    }

    println!("\nNumber at Index {}: {}", num_map[&prev_num].0, prev_num);
}
