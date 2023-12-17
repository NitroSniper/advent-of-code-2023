pub fn problem(input: &str) -> String {
    input
        .split_terminator('\n')
        .map(|line| {
            10 * line
                .chars()
                .nth(line.find(char::is_numeric).expect("a number"))
                .expect("THERE IS A NUMBER")
                .to_digit(10)
                .expect("it will happen")
                + line
                    .chars()
                    .nth(line.rfind(char::is_numeric).expect("a number"))
                    .expect("THERE IS A NUMBER")
                    .to_digit(10)
                    .expect("it will happen")
        })
        .sum::<u32>().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = problem(include_str!("./data/test_1"));
        assert_eq!(result, "142")
    }
}
