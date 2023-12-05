#[derive(Debug)]
struct Instruction {
    num: usize,
    src: usize,
    dst: usize,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let ["move", num, "from", src, "to", dst] = value.split(' ').collect::<Vec<&str>>()[..]
        else {
            panic!()
        };
        Instruction {
            num: num.parse().unwrap(),
            src: src.parse().unwrap(),
            dst: dst.parse().unwrap(),
        }
    }
}

impl Instruction {
    fn execute(&self, stacks: &mut Vec<Vec<char>>) {
        let mut temp = vec![];
        for _ in 0..self.num {
            temp.push(stacks[self.src - 1].pop().unwrap());
        }
        temp.iter().rev().for_each(|x| stacks[self.dst - 1].push(*x))
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let [stacks, instructions] = input.split("\n\n").collect::<Vec<_>>()[..] else {
        panic!()
    };
    let mut stacks = parse_initial_stacks(stacks);
    let instructions = instructions
        .split('\n')
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(Instruction::from)
        .collect::<Vec<_>>();

    instructions.iter().for_each(|x| x.execute(&mut stacks));
    println!(
        "{:#?}",
        stacks
            .iter_mut()
            .map(|x| x.pop().unwrap())
            .collect::<String>()
    );
}

fn parse_initial_stacks(s: &str) -> Vec<Vec<char>> {
    let lines: Vec<_> = s.split('\n').filter(|x| !x.is_empty()).rev().collect();
    let num_stacks = lines[0].trim().chars().rev().nth(0).unwrap();
    let mut initial_stacks: Vec<Vec<char>> =
        vec![vec![]; (num_stacks as u32 - '0' as u32) as usize];

    lines.iter().skip(1).for_each(|x| {
        x.chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .enumerate()
            .for_each(|(index, item)| match item[1] {
                ' ' => {}
                char => initial_stacks[index].push(char),
            })
    });
    initial_stacks
}
