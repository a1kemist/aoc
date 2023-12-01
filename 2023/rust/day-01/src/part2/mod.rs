
pub fn solve(input: &str) -> Result<String, String> {
    let result = input.lines().fold(
        0,
        |mut agg: u32, line: &str| -> u32 {
            agg += parse_line(line);
            agg
        }
    );

    Ok(result.to_string())
}

fn parse_line(line: &str) -> u32 {
    // assume ascii chars
    let line_bytes = line.as_bytes();

    let mut index: usize = 0;
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    'outer: while index < line.len() {
        let c = line_bytes[index];
        if first.is_none() {
            // check numerics first
            if c.is_ascii_digit() {
                first = (c as char).to_digit(10).map(|x| x * 10);
                last = (c as char).to_digit(10);
                index += 1;
                continue 'outer;
            }
            else {
                // check every token
                for (pos, TOKEN) in TOKENS.iter().enumerate() {
                    let upper_bound = index + TOKEN.len();
                    if upper_bound <= line_bytes.len() {
                        let sub = &line_bytes[index..upper_bound];
                        if sub.eq((*TOKEN).as_bytes()) {
                            let num: u32 = (pos + 1) as u32;
                            first = Some(num * 10);
                            last = Some(num);
                            // note that we only advance to before the last char of TOKEN
                            // because we need to account for overlapping tokens and none
                            // of the tokens overlap past the first/last chars
                            let advance = TOKEN.len() - 1;
                            index += advance;
                            continue 'outer;
                        }
                    }
                }
            }
        }
        else {
            // check numerics first
            if c.is_ascii_digit() {
                last = (c as char).to_digit(10);
                index += 1;
                continue 'outer;
            }
            else {
                // check every token
                for (pos, TOKEN) in TOKENS.iter().enumerate() {
                    let upper_bound = index + TOKEN.len();
                    if upper_bound <= line_bytes.len() {
                        let sub = &line_bytes[index..upper_bound];
                        if sub.eq((*TOKEN).as_bytes()) {
                            let num: u32 = (pos + 1) as u32;
                            last = Some(num);
                            // note that we only advance to before the last char of TOKEN
                            // because we need to account for overlapping tokens and none
                            // of the tokens overlap past the first/last chars
                            let advance = TOKEN.len() - 1;
                            index += advance;
                            continue 'outer;
                        }
                    }
                }
            }
        }

        index += 1;
    }

    let result = first.unwrap_or(0) + last.unwrap_or(0);
    result
}

const TOKENS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

#[cfg(test)]
mod tests {
    use crate::part2::solve;
    use crate::part2::parse_line;

    #[test]
    fn test_solve() -> Result<(), String> {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!("281", solve(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_line() -> Result<(), String> {
        let line_tests = [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
            // extra examples for overlapping "word" tokens
            ("oneight", 18),
            ("twoneight", 28),
            ("eighthree", 83),
            ("1eighthree", 13),
        ];
        line_tests.iter().for_each(|(line, calibration)| {
            let parsed = parse_line(line);
            assert_eq!(*calibration, parsed);
        });
        Ok(())
    }
}