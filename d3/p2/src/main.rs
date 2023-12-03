use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Group {
    rucksack_1: HashSet<char>,
    rucksack_2: HashSet<char>,
    rucksack_3: HashSet<char>,
}

impl From<&[&str]> for Group {
    fn from(value: &[&str]) -> Self {
        Group {
            rucksack_1: value[0].chars().collect(),
            rucksack_2: value[1].chars().collect(),
            rucksack_3: value[2].chars().collect(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let items = input
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(Group::from)
        .map(get_badge)
        .map(get_priority)
        .map(|x| x as u32)
        .sum::<u32>();

    println!("{:#?}", items);
}

fn get_badge(group: Group) -> char {
    let badges: HashSet<_> = group.rucksack_1
        .intersection(&group.rucksack_2)
        .copied()
        .collect::<HashSet<_>>()
        .intersection(&group.rucksack_3)
        .copied()
        .collect();
    assert_eq!(badges.len(), 1);
    badges.iter().nth(0).unwrap().to_owned()
}

fn get_priority(char: char) -> u8 {
    match char {
        c if c.is_lowercase() => (c as u8) - ('a' as u8) + 1,
        c if c.is_uppercase() => (c as u8) - ('A' as u8) + 1 + 26,
        _ => panic!(),
    }
}
