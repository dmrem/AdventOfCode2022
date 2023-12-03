use std::collections::HashSet;
use std::fs;

struct Pairs {
    assignment_1: HashSet<u16>,
    assignment_2: HashSet<u16>,
}

impl From<&str> for Pairs {
    fn from(value: &str) -> Self {
        let sections: Vec<_> = value.split(',').collect();
        assert_eq!(sections.len(), 2);
        let a1_bounds = sections[0].split('-').map(|x| x.parse::<u16>()).map(Result::unwrap).collect::<Vec<_>>();
        let a2_bounds = sections[1].split('-').map(|x| x.parse::<u16>()).map(Result::unwrap).collect::<Vec<_>>();
        Pairs {
            assignment_1: (a1_bounds[0]..=a1_bounds[1]).collect(),
            assignment_2: (a2_bounds[0]..=a2_bounds[1]).collect(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();
    println!(
        "{}",
        lines
            .iter()
            .map(|x| Pairs::from(*x))
            .filter(|x| !x.assignment_1.is_disjoint(&x.assignment_2))
            .count()
    );
}
