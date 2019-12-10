use common::Puzzle;
use common::computer::{Computer, Tape, Word};

fn main() {
    let mut a: Puzzle1 =  Default::default();
    a.run();

    let mut b: Puzzle2 =  Default::default();
    b.run();
}

#[derive(Default)]
struct Puzzle1 {
    result: Word
}

impl Puzzle1 {
    fn run_tape(in_a: Word, in_b: Word, in_tape: &Tape) -> Word {
        let mut computer = Computer::new_with_tape(in_tape);
        computer.memory.write_direct(1, in_a);
        computer.memory.write_direct(2, in_b);
        return computer.run();
    }
}

impl Puzzle for Puzzle1 {
    type ParsedLine = Tape;

    fn process_item(&mut self, item: Self::ParsedLine) {
        self.result = Puzzle1::run_tape(12, 2, &item);
    }

    fn final_result(&mut self) -> String {
        self.result.to_string()
    }
}

#[derive(Default)]
struct Puzzle2 {
    noun: Word,
    verb: Word
}

impl Puzzle for Puzzle2 {
    type ParsedLine = Tape;

    fn process_item(&mut self, item: Self::ParsedLine) {
        for noun in 0 .. 100 {
            for verb in 0 .. 100 {
                let result = Puzzle1::run_tape(noun, verb, &item);
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
