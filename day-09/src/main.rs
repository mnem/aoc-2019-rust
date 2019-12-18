use common::computer::{Computer, Tape};
use common::Puzzle;

fn main() {
    let mut a = Puzzle1 { output: String::new() };
    a.run();

    let mut b = Puzzle2 { output: String::new() };
    b.run();
}

struct Puzzle1 {
    output: String,
}

impl Puzzle for Puzzle1 {
    type ParsedLine = Tape;

    fn process_item(&mut self, item: Self::ParsedLine) {
        let mut computer = Computer::new_with_tape(&item);
        computer.io.add_input(1);
        computer.run();
        let s: Vec<String> = computer.io.output.iter().map( |n| n.to_string() ).into_iter().collect();
        self.output = s.join(",");
    }

    fn final_result(&mut self) -> String {
        self.output.clone()
    }
}

struct Puzzle2 {
    output: String,
}

impl Puzzle for Puzzle2 {
    type ParsedLine = Tape;

    fn process_item(&mut self, item: Self::ParsedLine) {
        let mut computer = Computer::new_with_tape(&item);
        computer.io.add_input(2);
        computer.run();
        let s: Vec<String> = computer.io.output.iter().map( |n| n.to_string() ).into_iter().collect();
        self.output = s.join(",");
    }

    fn final_result(&mut self) -> String {
        self.output.clone()
    }
}
