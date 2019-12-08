pub mod computer;

use std::fs;

pub trait Puzzle {
    type ParsedLine: std::str::FromStr;

    fn process_item(&mut self, item: Self::ParsedLine);
    fn final_result(&mut self) -> String;

    fn parse_line(&mut self, line: &str) -> Self::ParsedLine {
        match line.parse() {
            Ok(i) => i,
            Err(_) => panic!()
        }
    }

    fn input(&self) -> String {
        let input_filename = String::from("input.txt");
        fs::read_to_string(input_filename)
            .expect("Failed to read file")
    }

    fn run(&mut self) {
        let input = self.input();
        for line in input.lines() {
            let trimmed = line.trim();
            if trimmed.len() == 0 {
                continue;
            }
            let item = self.parse_line(line);
            self.process_item(item);
        }
        println!("Result: {}\n", self.final_result());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct IntegerAdder {
        total: i64
    }

    impl Puzzle for IntegerAdder {
        type ParsedLine = i64;

        fn process_item(&mut self, item: Self::ParsedLine) {
            self.total += item;
        }

        fn final_result(&mut self) -> String {
            self.total.to_string()
        }

        fn input(&self) -> String {
            String::from("1\n2\n3\n")
        }
    }

    struct StringCatter {
        result: String
    }

    impl Puzzle for StringCatter {
        type ParsedLine = String;

        fn process_item(&mut self, item: Self::ParsedLine) {
            self.result.push_str(&item);
        }

        fn final_result(&mut self) -> String {
            self.result.clone()
        }
    }

    #[test]
    fn test_input_override() {
        let mut subject: IntegerAdder = Default::default();
        subject.run();
        assert_eq!("6", subject.final_result());
    }

    #[test]
    fn test_input() {
        let mut subject = StringCatter { result: String::new() };
        subject.run();
        assert_eq!("abcde", subject.final_result());
    }
}
