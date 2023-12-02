use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let x = file
        .split("\r\n\r\n")
        .map(|x| {
            x.split("\r\n")
                .filter_map(|x| x.trim().parse::<i32>().ok())
                .sum::<i32>()
        })
        .max()
        .unwrap();
    println!("{:?}", x);
}
