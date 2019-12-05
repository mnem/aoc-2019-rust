use common::Puzzle;

fn main() {
    let a = PuzzleRange { lower: 264793, upper: 803935 };
    println!("Possibilities A: {}", a.count_possibilities_a() );
    println!("Possibilities B: {}", a.count_possibilities_b() );
}

struct PuzzleRange {
    lower: i32,
    upper: i32
}

impl PuzzleRange {
    fn is_valid(&self, n: i32, strict_pairs: bool) -> bool {
        let as_str = n.to_string();

        for i in 1 .. as_str.len() {
            let a = as_str.chars().nth(i-1).unwrap();
            let b = as_str.chars().nth(i).unwrap();
            if a > b {
                return false;
            }
        }

        let mut has_identical_pair = false;
        let mut current = 0;
        while current < as_str.len() - 1 {
            let a = as_str.chars().nth(current).unwrap();
            let b = as_str.chars().nth(current + 1).unwrap();
            if a == b {
                if strict_pairs && current < as_str.len() - 2 {
                    let c = as_str.chars().nth(current + 2).unwrap();
                    if b == c {
                        current += 3;
                        while current < as_str.len() - 1 && a == as_str.chars().nth(current).unwrap() {
                            current += 1;
                        }
                        continue;
                    }
                }
                has_identical_pair = true;
                break;
            }
            current += 1;
        }
        if has_identical_pair == false {
            return false;
        }

        return true;
    }

    fn count_possibilities_a(&self) -> i32 {
        let mut possibilities = 0;
        for i in self.lower ..= self.upper {
            if self.is_valid(i, false) {
                possibilities += 1;
            }
        }
        return possibilities;
    }

    fn count_possibilities_b(&self) -> i32 {
        let mut possibilities = 0;
        for i in self.lower ..= self.upper {
            if self.is_valid(i, true) {
                possibilities += 1;
            }
        }
        return possibilities;
    }
}
