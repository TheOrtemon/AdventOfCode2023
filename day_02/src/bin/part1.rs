use regex::Regex;

fn is_valid_game(line: &str) -> bool {
    let re = Regex::new(r"(\d+) (red|green|blue)(:?[,;] )?").unwrap();
    for groups in re.captures_iter(line) {
        let color = groups.get(2).unwrap().as_str();
        let count = groups.get(1).unwrap().as_str().parse::<u32>().unwrap();
        match color {
            "red" => if count > 12 {return false},
            "green" => if count > 13 {return false},
            "blue" => if count > 14 {return false},
            _ => unreachable!()
        }
    }
    true
}

fn count_ids(s: &str) -> u32 {
    s
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let line = line.split(": ").last().unwrap();
            if is_valid_game(line) {i as u32 + 1} else {0}
        }) 
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let res = count_ids(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(count_ids(test_input), 8);
    }
}