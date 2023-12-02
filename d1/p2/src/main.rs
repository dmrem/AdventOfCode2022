use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut x = file
        .split("\r\n\r\n")
        .map(|x| {
            x.split("\r\n")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    x.sort();

    println!("{:?}", x.iter().rev().take(3).sum::<i32>());
}
