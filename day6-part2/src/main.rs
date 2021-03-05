fn find_common(vec1: Vec<char>, vec2: Vec<char>) -> Vec<char> {
    let mut return_vec: Vec<char> = Vec::new();
    for c in vec1 {
        if vec2.contains(&c) {
            return_vec.push(c);
        }
    }

    return_vec
}

fn main() {
    let group_iter = include_str!("../input.txt").split("\n\n");

    let mut sum: u32 = 0;
    for group in group_iter {
        let mut line_iter = group.trim().split('\n');
        let mut common_chars: Vec<char> = line_iter.next().unwrap().chars().collect::<Vec<char>>();

        for line in line_iter {
            let line_chars = line.chars().collect::<Vec<char>>();

            common_chars = find_common(common_chars, line_chars);
        }

        sum += common_chars.len() as u32;
    }

    println!("Sum: {}", sum);
}
