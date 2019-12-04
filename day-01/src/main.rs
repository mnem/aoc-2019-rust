use common::Puzzle;

fn main() {
    let mut a: Puzzle1 = Default::default();
    a.run();

    let mut b: Puzzle2 = Default::default();
    b.run();
}

#[derive(Default)]
struct Puzzle1 {
    total: i64
}

impl Puzzle1 {
    fn fuel_for_mass(item: i64) -> i64 {
        item / 3 - 2
    }
}

impl Puzzle for Puzzle1 {
    type ParsedLine = i64;

    fn process_item(&mut self, item: Self::ParsedLine) {
        self.total += Puzzle1::fuel_for_mass(item)
    }

    fn final_result(&mut self) -> String {
        self.total.to_string()
    }
}

#[derive(Default)]
struct Puzzle2 {
    total: i64
}

impl Puzzle2 {
    fn fuel_for_fuel(fuel: i64) -> i64 {
        let mut remaining_mass = Puzzle1::fuel_for_mass(fuel);
        let mut extra_fuel: i64 = 0;
        while remaining_mass > 0 {
            extra_fuel += remaining_mass;
            remaining_mass = Puzzle1::fuel_for_mass(remaining_mass);
        }
        return extra_fuel;
    }
}

impl Puzzle for Puzzle2 {
    type ParsedLine = i64;

    fn process_item(&mut self, item: Self::ParsedLine) {
        let primary_fuel = Puzzle1::fuel_for_mass(item);
        let secondary_fuel = Puzzle2::fuel_for_fuel(primary_fuel);
        self.total += primary_fuel + secondary_fuel;
    }

    fn final_result(&mut self) -> String {
        self.total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_one() {
        let mut subject: Puzzle1 = Default::default();
        subject.run();
        assert_eq!(3282935, subject.total);
    }

    #[test]
    fn test_puzzle_two_a() {
        let fuel: i64 = 654;
        let result = Puzzle2::fuel_for_fuel(fuel);
        assert_eq!(312, result);
        assert_eq!(966, result + fuel);
    }

    #[test]
    fn test_puzzle_two_b() {
        let fuel: i64 = 33583;
        let result = Puzzle2::fuel_for_fuel(fuel);
        assert_eq!(50346, result + fuel);
    }
}
