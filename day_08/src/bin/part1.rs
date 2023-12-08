use std::collections::HashMap;

fn walk_the_way(s: &str) -> usize {
    let mut lines = s.lines().filter(|line| !line.is_empty());
    let path = lines.next().unwrap();
    let way_map: HashMap<&str, (&str, &str)> = lines.map(|line| {
        let mut parts = line.split(" = ");
        let destination = parts.next().unwrap();
        let ways = parts.last().unwrap();
        let trimmed_ways: Vec<&str> = ways[1..ways.len() - 1].split(", ").collect();
        let [first_way, last_way] = &trimmed_ways[0..2] else { unreachable!() };
        (destination, (*first_way, *last_way))
    }).collect();

    let mut cur_destination = "AAA";

    for (steps, char) in path.chars().cycle().enumerate() {
        let ways = way_map.get(cur_destination).unwrap();
        match char {
            'L' => cur_destination = ways.0,
            'R' => cur_destination = ways.1,
            _ => unreachable!()
        }
        if cur_destination == "ZZZ" {
            return steps + 1
        }
    }
    unreachable!()
}

fn main() {
    let input = include_str!("./input.txt");
    let res = walk_the_way(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(walk_the_way(test_input), 6);
    }
}