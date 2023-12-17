#[derive(Debug)]
struct NumberLocation {
    location: usize,
    number: u32,
}

impl NumberLocation {
    ///
    /// Returns the NumberLocation of the first number found in line using the find Function
    ///
    /// Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html) if the pattern doesn't match.
    fn find(line: &str, find_function: impl Fn(&str) -> Option<usize>) -> Option<Self> {
        let index = find_function(line)?;
        unsafe {
            Some(NumberLocation {
                location: index,
                number: line
                    .chars()
                    .nth(index)
                    .unwrap_unchecked()
                    .to_digit(10)
                    .unwrap_unchecked(),
            })
        }
    }
}

pub fn problem(input: &str) -> String {
    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .split_terminator('\n')
        .map(|line| {
            let l_loc = NumberLocation::find(line, |l| l.find(|c: char| c.is_ascii_digit()));
            let r_loc = NumberLocation::find(line, |l| l.rfind(|c: char| c.is_ascii_digit()));
            let (l_loc, r_loc) = numbers.into_iter().enumerate().fold(
                (l_loc, r_loc),
                |(mut l_loc, mut r_loc), (num, num_as_str)| {
                    if let Some(index) = line.rfind(num_as_str) {
                        if !r_loc
                            .as_ref()
                            .is_some_and(|num_loc| num_loc.location > index)
                        {
                            r_loc = Some(NumberLocation {
                                location: index,
                                number: num as u32,
                            })
                        }
                    }
                    if let Some(index) = line.find(num_as_str) {
                        if !l_loc
                            .as_ref()
                            .is_some_and(|num_loc| num_loc.location < index)
                        {
                            l_loc = Some(NumberLocation {
                                location: index,
                                number: num as u32,
                            })
                        }
                    }
                    (l_loc, r_loc)
                },
            );
            l_loc.expect("it").number * 10 + r_loc.expect("it").number
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = problem(include_str!("./data/test_2"));
        assert_eq!(result, "281")
    }
}
