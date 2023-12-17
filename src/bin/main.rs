use std::env;

fn main() {
    let input = include_str!("../data/input");
    let result = match env::args()
        .nth(1)
        .expect("Please add an argument of either '1' OR '2' for part 1 / 2 respectively ")
        .as_str()
    {
        "1" => library::part1::problem(input),
        "2" => library::part2::problem(input),
        _ => panic!("This is not a valid argument"),
    };
    dbg!(result);
}
