use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    for i in 14..input.len() {
        if input[i-14..i].chars().collect::<HashSet<char>>().len() == 14 {
            println!("{}", i);
            break;
        }
    }
}
