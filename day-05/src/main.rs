use common::Puzzle;
use common::computer::{Computer, Tape};

fn main() {
    let mut a = Puzzle1 { result: 0, output: None };
    a.run();

    let mut b = Puzzle2 { result: 0, output: None };
    b.run();
}

struct Puzzle1 {
    result: i32,
    output: Option<Vec<i32>>,
}

impl Puzzle for Puzzle1 {
    type ParsedLine = Tape;

    fn process_item(&mut self, item: Self::ParsedLine) {
        let mut computer = Computer::new_with_tape(&item);
        computer.io.add_input(1);
        self.result = computer.run();
        self.output = Some(computer.io.output.clone());
    }

    fn final_result(&mut self) -> String {
        self.output.as_ref().unwrap().last().unwrap().to_string()
    }
}

struct Puzzle2 {
    result: i32,
    output: Option<Vec<i32>>,
}

impl Puzzle for Puzzle2 {
    type ParsedLine = Tape;

    fn process_item(&mut self, item: Self::ParsedLine) {
        let mut computer = Computer::new_with_tape(&item);
        computer.io.add_input(5);
        self.result = computer.run();
        self.output = Some(computer.io.output.clone());
    }

    fn final_result(&mut self) -> String {
        self.output.as_ref().unwrap().last().unwrap().to_string()
    }
}
