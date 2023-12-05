use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    for i in 4..input.len() {
        if input[i-4..i].chars().collect::<HashSet<char>>().len() == 4 {
            println!("{}", i);
            break;
        }
    }
}
