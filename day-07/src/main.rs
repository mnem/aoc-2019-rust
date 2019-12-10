use permutohedron::heap_recursive;
use common::Puzzle;
use common::computer::{Computer, Tape, CPUState, Word};

fn main() {
    let mut a = Puzzle1 { result: 0 };
    a.run();

    let mut b = Puzzle2 { result: 0 };
    b.run();
}

struct Puzzle1 {
    result: Word,
}

impl Puzzle for Puzzle1 {
    type ParsedLine = Tape;

    fn process_item(&mut self, item: Self::ParsedLine) {
        let mut amp_a = Computer::new();
        let mut amp_b = Computer::new();
        let mut amp_c = Computer::new();
        let mut amp_d = Computer::new();
        let mut amp_e = Computer::new();

        let mut inputs = vec![0,1,2,3,4];
        let mut permutations = Vec::new();
        heap_recursive(&mut inputs, |permutation| {
            permutations.push(permutation.to_vec())
        });

        let mut max = 0;
        for input in permutations {
            // Set the phase settings
            amp_a.reset_and_load_tape(&item);
            amp_a.io.add_input(input[0]);

            amp_b.reset_and_load_tape(&item);
            amp_b.io.add_input(input[1]);

            amp_c.reset_and_load_tape(&item);
            amp_c.io.add_input(input[2]);

            amp_d.reset_and_load_tape(&item);
            amp_d.io.add_input(input[3]);

            amp_e.reset_and_load_tape(&item);
            amp_e.io.add_input(input[4]);

            // Run the first amp
            amp_a.io.add_input(0);
            amp_a.run();

            // Chain the remaining ones
            amp_b.io.add_input(*amp_a.io.output.first().unwrap());
            amp_b.run();

            amp_c.io.add_input(*amp_b.io.output.first().unwrap());
            amp_c.run();

            amp_d.io.add_input(*amp_c.io.output.first().unwrap());
            amp_d.run();

            amp_e.io.add_input(*amp_d.io.output.first().unwrap());
            amp_e.run();

            // Check the output
            max = max.max(*amp_e.io.output.first().unwrap());
        }
        self.result = max;
    }

    fn final_result(&mut self) -> String {
        self.result.to_string()
    }
}

struct Puzzle2 {
    result: Word,
}

impl Puzzle for Puzzle2 {
    type ParsedLine = Tape;

    fn process_item(&mut self, item: Self::ParsedLine) {
        let mut amp_a = Computer::new();
        let mut amp_b = Computer::new();
        let mut amp_c = Computer::new();
        let mut amp_d = Computer::new();
        let mut amp_e = Computer::new();

        let mut inputs = vec![5,6,7,8,9];
        let mut permutations = Vec::new();
        heap_recursive(&mut inputs, |permutation| {
            permutations.push(permutation.to_vec())
        });

        let mut max = 0;
        for input in permutations {
            // Set the phase settings
            amp_a.reset_and_load_tape(&item);
            amp_a.io.add_input(input[0]);

            amp_b.reset_and_load_tape(&item);
            amp_b.io.add_input(input[1]);

            amp_c.reset_and_load_tape(&item);
            amp_c.io.add_input(input[2]);

            amp_d.reset_and_load_tape(&item);
            amp_d.io.add_input(input[3]);

            amp_e.reset_and_load_tape(&item);
            amp_e.io.add_input(input[4]);

            // Run the first amp
            amp_a.io.add_input(0);
            loop {
                amp_a.run();

                while amp_a.io.output.len() > 0 {
                    amp_b.io.add_input(amp_a.io.output.pop().unwrap());
                }

                amp_b.run();

                while amp_b.io.output.len() > 0 {
                    amp_c.io.add_input(amp_b.io.output.pop().unwrap());
                }

                amp_c.run();

                while amp_c.io.output.len() > 0 {
                    amp_d.io.add_input(amp_c.io.output.pop().unwrap());
                }

                amp_d.run();

                while amp_d.io.output.len() > 0 {
                    amp_e.io.add_input(amp_d.io.output.pop().unwrap());
                }

                amp_e.run();

                if amp_e.cpu_state() == CPUState::Halted {
                    max = max.max(amp_e.io.output.pop().unwrap());
                    break;
                } else {
                    assert!(amp_e.io.output.len() > 0);
                    while amp_e.io.output.len() > 0 {
                        amp_a.io.add_input(amp_e.io.output.pop().unwrap());
                    }
                }
            }
        }
        self.result = max;
    }

    fn final_result(&mut self) -> String {
        self.result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let tape = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".parse().unwrap();
        let mut puzzle = Puzzle1 { result: 0 };
        puzzle.process_item(tape);
        assert_eq!(43210, puzzle.result);
    }

    #[test]
    fn example_2() {
        let tape = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".parse().unwrap();
        let mut puzzle = Puzzle1 { result: 0 };
        puzzle.process_item(tape);
        assert_eq!(54321, puzzle.result);
    }

    #[test]
    fn example_3() {
        let tape = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".parse().unwrap();
        let mut puzzle = Puzzle1 { result: 0 };
        puzzle.process_item(tape);
        assert_eq!(65210, puzzle.result);
    }

    #[test]
    fn example_2_1() {
        let tape = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5".parse().unwrap();
        let mut puzzle = Puzzle2 { result: 0 };
        puzzle.process_item(tape);
        assert_eq!(139629729, puzzle.result);
    }

    #[test]
    fn example_2_2() {
        let tape = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10".parse().unwrap();
        let mut puzzle = Puzzle2 { result: 0 };
        puzzle.process_item(tape);
        assert_eq!(18216, puzzle.result);
    }
}
