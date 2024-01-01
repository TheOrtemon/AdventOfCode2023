use std::collections::BTreeSet;

fn explore_map(input: &str) -> usize {
    let directions: [[isize; 2]; 4] = [
        [0, -1],
        [-1, 0],
        [0, 1],
        [1, 0]
    ];
    let map: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let start = map.iter()
        .enumerate()
        .find_map(|(y, row)| row.iter()
            .enumerate()
            .find_map(|(x, c)| if *c == 'S' {Some([x, y])} else {None})
        ).unwrap();
    let mut prev_round_set = BTreeSet::from([start]);
    let mut next_round_set = BTreeSet::new();
    for _ in 1..=6 {
        while let Some([x, y]) = prev_round_set.pop_first() {
            for [dx, dy] in directions.iter() {
                let [xi, yi] = [x.wrapping_add_signed(*dx), y.wrapping_add_signed(*dy)];
                let isnt_wall = map
                    .get(yi)
                    .and_then(|row| row.get(xi))
                    .filter(|&c| *c != '#')
                    .is_some();
                if isnt_wall {
                    next_round_set.insert([xi, yi]);
                }
            }
        }
        (prev_round_set, next_round_set) = (next_round_set, prev_round_set);
    }
    prev_round_set.len()
}

fn main() {
    let input = include_str!("./input.txt");
    let res  = explore_map(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(explore_map(test_input), 16);
    }
}
