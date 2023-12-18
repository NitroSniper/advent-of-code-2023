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

    fn make_enough(&mut self, other: &Self) {
        if self.red < other.red {
            self.red = other.red
        }
        if self.green < other.green {
            self.green = other.green;
        }
        if self.blue < other.blue {
            self.blue = other.blue;
        }
    }
    fn make_power(&self) -> u32 {
        self.red * self.green * self.blue
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

fn update_actual_bag(set: &str, actual_bag: &mut Bag) {
    let mut game_bag = Bag::default();
    for cubes in set.split_terminator(',') {
        let (number, color) = cubes.trim().split_once(' ').unwrap();
        game_bag.add_cube(
            Color::from_str(color).expect("to turn to enum"),
            number.parse::<u32>().expect("to turn to number"),
            )
    }
    actual_bag.make_enough(&game_bag)
}
pub fn problem(input: &str) -> String {
    let mut total = 0;
    for line in input.split_terminator('\n') {
        let mut actual_bag = Bag::default();
        let (_, cube_set) = line.split_once(':').unwrap();
        for set in cube_set.split_terminator(';') {
            update_actual_bag(set, &mut actual_bag)
        }
        total += actual_bag.make_power();
    }
    total.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = problem(include_str!("./data/test_2"));
        assert_eq!(result, "2286")
    }
}
