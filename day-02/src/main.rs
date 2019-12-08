use common::Puzzle;
use common::computer::Computer;
use std::str::FromStr;

fn main() {
    let mut a: Puzzle1 =  Default::default();
    a.run();

    let mut b: Puzzle2 =  Default::default();
    b.run();
}

#[derive(Default)]
struct Puzzle1 {
    result: i32
}

#[derive(Debug, PartialEq, Clone)]
struct PuzzleData {
    data: Vec<i32>
}

impl FromStr for PuzzleData {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split(",").map(|i| i.parse::<i32>().unwrap() ).collect();
        Ok(PuzzleData { data: data } )
    }
}

impl Puzzle1 {
    fn run_tape(in_a: i32, in_b: i32, in_tape: &Vec<i32>) -> i32 {
        let mut computer = Computer::new_with_tape(in_tape);
        computer.memory.write(1, in_a);
        computer.memory.write(2, in_b);
        return computer.run();
    }
}

impl Puzzle for Puzzle1 {
    type ParsedLine = PuzzleData;

    fn process_item(&mut self, item: Self::ParsedLine) {
        self.result = Puzzle1::run_tape(12, 2, &item.data);
    }

    fn final_result(&mut self) -> String {
        self.result.to_string()
    }
}

#[derive(Default)]
struct Puzzle2 {
    noun: i32,
    verb: i32
}

impl Puzzle for Puzzle2 {
    type ParsedLine = PuzzleData;

    fn process_item(&mut self, item: Self::ParsedLine) {
        for noun in 0 .. 100 {
            for verb in 0 .. 100 {
                let result = Puzzle1::run_tape(noun, verb, &item.data);
                if result == 19690720 {
                    self.noun = noun;
                    self.verb = verb;
                    return;
                }
            }
        }
    }

    fn final_result(&mut self) -> String {
        (100 * self.noun + self.verb).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1_works() {
        let mut a: Puzzle1 =  Default::default();
        a.run();
        assert_eq!(3931283, a.result);
    }

    #[test]
    fn test_puzzle2_works() {
        let mut b: Puzzle2 =  Default::default();
        b.run();
        assert_eq!(6979.to_string(), b.final_result());
    }
}
