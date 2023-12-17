#[derive(Debug)]
struct NumberLocation {
    location: usize,
    number: u32,
}

pub fn problem(input: &str) -> String {
    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut total = 0;
    for line in input.split_terminator('\n') {
        let mut r_loc: Option<NumberLocation> = None;
        if let Some(index) = line.rfind(|c: char| c.is_ascii_digit()) {
            let r = line.chars().nth(index).unwrap().to_digit(10).unwrap();
            r_loc = Some(NumberLocation {
                location: index,
                number: r,
            });
        }
        let mut l_loc: Option<NumberLocation> = None;
        if let Some(index) = line.find(|c: char| c.is_ascii_digit()) {
            let l = line.chars().nth(index).unwrap().to_digit(10).unwrap();
            l_loc = Some(NumberLocation {
                location: index,
                number: l,
            });
        }
        dbg!(line);
        for (number, number_as_str) in numbers.into_iter().enumerate() {
            if let Some(index) = line.rfind(number_as_str) {
                /*
                 * If r_loc is Some AND > than index then keep it. otherwise remove it.
                 * if r_loc is None OR r_loc < index then replace it
                 */

                if !r_loc
                    .as_ref()
                    .is_some_and(|num_loc| num_loc.location > index)
                {
                    r_loc = Some(NumberLocation {
                        location: index,
                        number: number as u32,
                    })
                }
            }

            if let Some(index) = line.find(number_as_str) {
                /*
                 * If l_loc is Some AND > than index then keep it. otherwise remove it.
                 * if l_loc is None OR r_loc < index then replace it
                 */

                if !l_loc
                    .as_ref()
                    .is_some_and(|num_loc| num_loc.location < index)
                {
                    l_loc = Some(NumberLocation {
                        location: index,
                        number: number as u32,
                    })
                }
            }
        }
        println!("{:?}", l_loc);
        println!("{:?}", r_loc);
        total += l_loc.expect("it").number*10 + r_loc.expect("it").number
    }
    total.to_string()
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
