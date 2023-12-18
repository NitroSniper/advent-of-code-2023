use std::str::FromStr;

#[derive(Debug, Default)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn add_cube(&mut self, color: Color, amount: u32) {
        match color {
            Color::Red => self.red += amount,
            Color::Green => self.green += amount,
            Color::Blue => self.blue += amount,
        };
    }

    fn is_enough(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }
}

enum Color {
    Red,
    Green,
    Blue,
}

impl std::str::FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "blue" => Ok(Color::Blue),
            "green" => Ok(Color::Green),
            _ => Err(()),
        }
    }
}

pub fn problem(input: &str) -> String {
    let actual_bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut total = 0;
    'line: for line in input.split_terminator('\n') {
        let (game_line, cube_set) = line.split_once(':').unwrap();
        let (_, game_number) = game_line.split_once(' ').unwrap();

        for set in cube_set.split_terminator(';') {
            let mut game_bag = Bag::default();
            for cubes in set.split_terminator(',') {
                let [number, color] = cubes.trim().split_terminator(' ').collect::<Vec<_>>()[..]
                else {
                    panic!("couldn't split up")
                };
                game_bag.add_cube(
                    Color::from_str(color).expect("to turn to enum"),
                    number.parse::<u32>().expect("to turn to number"),
                )
            }

            if !actual_bag.is_enough(&game_bag) {
                continue 'line;
            }
        }
        total += game_number.parse::<u32>().expect("this to be a number");
    }
    total.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = problem(include_str!("./data/test_1"));
        assert_eq!(result, "8")
    }
}
