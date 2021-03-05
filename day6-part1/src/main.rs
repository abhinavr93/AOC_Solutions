fn main() {
    let group_iter = include_str!("../input.txt").split("\n\n");

    let mut sum: u32 = 0;
    for group in group_iter {
        let mut unique_chars: Vec<char> = Vec::new();
        for c in group.chars() {
            if c != '\n' && !unique_chars.contains(&c) {
                unique_chars.push(c);
            }
        }

        sum += unique_chars.len() as u32;
    }

    println!("Sum: {}", sum);
}
