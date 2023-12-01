// use crate::error::AocError;

pub fn solve(input: &str) -> Result<String, String> {
    let result = input.lines().fold(
        0,
        |mut agg: u32, line: &str| -> u32 {
            let mut it = line.chars();
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;
            while let Some(c) = it.next() {
                if c.is_ascii_digit() {
                    if first.is_none() {
                        first = c.to_digit(10).map(|x| x * 10);
                        last = c.to_digit(10);
                    }
                    else {
                        last = c.to_digit(10);
                    }
                }
            }
            let calibration = first.unwrap_or(0) + last.unwrap_or(0);
            agg += calibration;
            agg
        }
    );
    
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use crate::part1::solve;

    #[test]
    fn test_solve() -> Result<(), String> {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!("142", solve(input)?);
        Ok(())
    }
}

