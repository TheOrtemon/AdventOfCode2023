use std::collections::HashMap;
use num::Integer;

fn walk_the_way(s: &str) -> usize {
    let mut lines = s.lines().filter(|line| !line.is_empty());
    let path = lines.next().unwrap();
    let way_map: HashMap<&str, [&str; 2]> = lines.map(|line| {
        let mut parts = line.split(" = ");
        let destination = parts.next().unwrap();
        let ways = parts.last().unwrap();
        let trimmed_ways: Vec<&str> = ways[1..ways.len() - 1].split(", ").collect();
        let [first_way, last_way] = &trimmed_ways[0..2] else { unreachable!() };
        (destination, [*first_way, *last_way])
    }).collect();

    let cur_destinations: Vec<&str> = way_map.keys().filter(|key| key.ends_with('A')).cloned().collect();

    let path_len = path.len();
    let counted_path = path.chars().cycle().enumerate();
    cur_destinations.iter().flat_map(|dest| {
        let mut cur_dest = *dest;
        let mut final_steps = vec![];
        let mut dest_to_step: HashMap<&str, Vec<usize>> = HashMap::new();
        'outer: for (steps, char) in counted_path.clone() {
            if let Some(final_dest) = dest_to_step.get(cur_dest) {
                for final_dest_i in final_dest {
                    if (final_dest_i % path_len) == (steps % path_len) {
                        break 'outer;
                    }
                }
            }
            dest_to_step.entry(cur_dest).or_default().push(steps);
            let side = match char {
                'L' => 0_usize,
                'R' => 1_usize,
                _ => unreachable!()
            }; 
            if cur_dest.ends_with('Z') {
                final_steps.push(steps);
            }
            cur_dest = way_map.get(cur_dest).unwrap()[side];
        }
        final_steps.into_iter()
    }).fold(1, |acc, cur| acc.lcm(&cur))
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
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(walk_the_way(test_input), 6);
    }
}