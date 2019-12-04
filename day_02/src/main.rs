use common::Puzzle;
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
        let mut tape = in_tape.clone();
        tape[1] = in_a;
        tape[2] = in_b;

        for i in (0 .. tape.len()).step_by(4) {
            let opcode = tape[i + 0];

            if opcode == 99 {
                break;
            }

            let a_addr = tape[i + 1] as usize;
            let b_addr = tape[i + 2] as usize;
            let dst_addr = tape[i + 3] as usize;

            let a = tape[a_addr];
            let b = tape[b_addr];

            if opcode == 1 {
                tape[dst_addr] = a + b;
            } else if opcode == 2 {
                tape[dst_addr] = a * b;
            } else {
                panic!("Something went wrong!");
            }
        }
        return tape[0];
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
