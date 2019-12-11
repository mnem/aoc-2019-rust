use common::Puzzle;
use std::str::FromStr;

fn main() {
    let mut a = Puzzle1 { result: 0, out_image: String::new() };
    a.run();
}

struct PuzzleInput {
    layers: Vec<String>,
}

impl PuzzleInput {
    fn width() -> usize {
        25
    }

    fn height() -> usize {
        6
    }
}

impl FromStr for PuzzleInput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunk = PuzzleInput::width() * PuzzleInput::height();
        let o = s.chars().collect::<Vec<char>>().chunks(chunk).map( |c| c.iter().cloned().collect::<String>() ).collect();

        Ok( PuzzleInput { layers: o } )
    }
}

struct Puzzle1 {
    result: usize,
    out_image: String,
}

impl Puzzle1 {
    fn count_digits(&self, string: &String, digit: &char) -> usize {
        string.chars().filter( |c| c == digit).count()
    }
}

impl Puzzle for Puzzle1 {
    type ParsedLine = PuzzleInput;

    fn process_item(&mut self, item: Self::ParsedLine) {
        let mut min = std::usize::MAX;
        let mut min_layer = 0usize;
        let mut layer_i = 0usize;
        for layer in &item.layers {
            let zeroes = self.count_digits(layer, &'0');
            if zeroes < min {
                min = zeroes;
                min_layer = layer_i;
            }
            layer_i += 1;
        }

        let ones = self.count_digits(&item.layers[min_layer], &'1');
        let twos = self.count_digits(&item.layers[min_layer], &'2');

        self.result = ones * twos;

        let image_size = PuzzleInput::width() * PuzzleInput::height();
//        self.out_image = std::iter::repeat(" ").take(image_size).collect();
        let mut o: Vec<char> = std::iter::repeat(' ').take(image_size).collect();
        for i in 0 .. image_size {
            for layer in &item.layers {
                let layer_pixel = layer.chars().nth(i).unwrap();
                if layer_pixel == '0' {
                    o[i] = ' ';
                    break;
                } else if layer_pixel == '1' {
                    o[i] = 'O';
                    break;
                }
            }
        }
        self.out_image = o.into_iter().collect();
    }

    fn final_result(&mut self) -> String {
        let mut s = String::new();
        for y in 0..PuzzleInput::height() {
            for x in 0..PuzzleInput::width() {
                s = s + &self.out_image.chars().nth(y * PuzzleInput::width() + x).unwrap().to_string();
            }
            s = s + "\n";
        }

        format!("checksum: {}\n{}", self.result, s)
    }
}
