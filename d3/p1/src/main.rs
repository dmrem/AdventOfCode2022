use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Rucksack {
    compartment_1: HashSet<char>,
    compartment_2: HashSet<char>,
}

impl From<&str> for Rucksack {
    fn from(value: &str) -> Self {
        assert_eq!(value.len() % 2, 0);

        Rucksack {
            compartment_1: value.chars().take(value.len() / 2).collect(),
            compartment_2: value.chars().skip(value.len() / 2).collect(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let items: u32 = input
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| Rucksack::from(x))
        .map(|x| get_bad_item(x))
        .map(|x| get_priority(x) as u32)
        .sum();
    println!("{:#?}", items);
}

fn get_bad_item(rucksack: Rucksack) -> char {
    let bad_items: HashSet<char> = rucksack
        .compartment_1
        .intersection(&rucksack.compartment_2)
        .copied()
        .collect();
    assert_eq!(bad_items.len(), 1);
    bad_items.iter().nth(0).unwrap().to_owned()
}

fn get_priority(char: char) -> u8 {
    match char {
        c if c.is_lowercase() => (c as u8) - ('a' as u8) + 1,
        c if c.is_uppercase() => (c as u8) - ('A' as u8) + 1 + 26,
        _ => panic!(),
    }
}
