use std::collections::HashMap;
use regex::Regex;


fn summator(input: &str) -> u32 {
    let word_to_number: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].into_iter().collect();
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    input.lines().map(|line| {
        let mut matches = vec![];

        for capture in re.captures_iter(line) {
            if let Some(inner_capture) = capture.get(0) {
                let start = inner_capture.start();
                let end = inner_capture.end();
    
                matches.push(&line[start..end]);
    
                if start + 1 < line.len() {
                    let next_capture = re.captures(&line[start + 1..]);
                    if let Some(next_capture) = next_capture {
                        if let Some(next_inner_capture) = next_capture.get(0) {
                            let next_start = start + 1 + next_inner_capture.start();
                            let next_end = start + 1 + next_inner_capture.end();
                            matches.push(&line[next_start..next_end]);
                        }
                    }
                }
            }
        }
        let digits: Vec<u32> = matches.iter().map(|s| {
            if s.len() > 1 {
                *word_to_number.get(s).unwrap()
            } else {
                s.parse::<u32>().unwrap()
            }
        }).collect();

        let (n1, n2) = (digits[0], digits.last().unwrap());
        let res = n1 * 10 + n2;
        println!("{res}");
        res
    }).sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let res = summator(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        twone3twone";
        assert_eq!(summator(test_input), 302);
    }
}