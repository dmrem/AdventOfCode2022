use std::fs;

#[derive(Debug)]
enum YourMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum OppMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Game {
    you: YourMove,
    opp: OppMove,
}

impl Game {
    fn get_score(self) -> i32 {
        match self {
            Game{ you: YourMove::Rock, opp: OppMove::Rock } => 3 + 1,
            Game{ you: YourMove::Rock, opp: OppMove::Paper } => 0 + 1,
            Game{ you: YourMove::Rock, opp: OppMove::Scissors } => 6 + 1,
            Game{ you: YourMove::Paper, opp: OppMove::Rock } => 6 + 2,
            Game{ you: YourMove::Paper, opp: OppMove::Paper } => 3 + 2,
            Game{ you: YourMove::Paper, opp: OppMove::Scissors } => 0 + 2,
            Game{ you: YourMove::Scissors, opp: OppMove::Rock } => 0 + 3,
            Game{ you: YourMove::Scissors, opp: OppMove::Paper } => 6 + 3,
            Game{ you: YourMove::Scissors, opp: OppMove::Scissors } => 3 + 3,
        }
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        Game {
            opp: match value.chars().nth(0) {
                Some('A') => OppMove::Rock,
                Some('B') => OppMove::Paper,
                Some('C') => OppMove::Scissors,
                _ => panic!(),
            },
            you: match value.chars().nth(2) {
                Some('X') => YourMove::Rock,
                Some('Y') => YourMove::Paper,
                Some('Z') => YourMove::Scissors,
                _ => panic!(),
            },
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let total_score: Vec<i32> = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| Game::from(x))
        .map(|x| x.get_score())
        .collect();

    println!("{:#?}", total_score.iter().sum::<i32>());
}

