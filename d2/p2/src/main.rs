use std::fs;

#[derive(Debug)]
enum RequiredResult {
    Lose,
    Draw,
    Win,
}

#[derive(Debug)]
enum OppMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Game {
    opp: OppMove,
    res: RequiredResult
}

impl Game {
    fn get_score(self) -> i32 {
        match self {
            Game{ opp: OppMove::Rock, res: RequiredResult::Lose } => 0 + 3,
            Game{ opp: OppMove::Rock, res: RequiredResult::Draw } => 3 + 1,
            Game{ opp: OppMove::Rock, res: RequiredResult::Win } => 6 + 2,
            Game{ opp: OppMove::Paper, res: RequiredResult::Lose } => 0 + 1,
            Game{ opp: OppMove::Paper, res: RequiredResult::Draw } => 3 + 2,
            Game{ opp: OppMove::Paper, res: RequiredResult::Win } => 6 + 3,
            Game{ opp: OppMove::Scissors, res: RequiredResult::Lose } => 0 + 2,
            Game{ opp: OppMove::Scissors, res: RequiredResult::Draw } => 3 + 3,
            Game{ opp: OppMove::Scissors, res: RequiredResult::Win } => 6 + 1,
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
            res: match value.chars().nth(2) {
                Some('X') => RequiredResult::Lose,
                Some('Y') => RequiredResult::Draw,
                Some('Z') => RequiredResult::Win,
                _ => panic!(),
            }
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

