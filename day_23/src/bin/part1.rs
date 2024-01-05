use std::collections::{HashSet, HashMap};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn matched_arrow(c: char) -> Direction {
        match c {
            '^' => Direction::Up,
            '<' => Direction::Left,
            'v' => Direction::Down,
            '>' => Direction::Right,
            _ => unreachable!(),
        }
    }
    fn coords(&self) -> [isize; 2] {
        match self {
            Direction::Up => [0, -1],
            Direction::Left => [-1, 0],
            Direction::Down => [0, 1],
            Direction::Right => [1, 0],
        }
    }
}

fn traverse_map(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (x_len, y_len) = (map[0].len(), map.len());
    let [start, finish] = [[1, 0], [x_len - 2, y_len - 1]];

    let directions = [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ];

    let mut paths = vec![(start, HashSet::new())];
    let mut global_trajectory: HashMap<[usize; 2], usize> = HashMap::new();
    let mut res = 0;

    while let Some(([x, y], trajectory)) = paths.pop() {
        if finish == [x, y] {
            res = res.max(trajectory.len());
            continue;
        }
        let directions_iter = directions.iter().filter(|&&dir| {
            let cur = map[y][x];
            cur == '.' || Direction::matched_arrow(cur) == dir
        });
        for dir in directions_iter {
            let mut new_trajectory = trajectory.clone();
            let [dx, dy] = dir.coords();
            let [xi, yi] = [x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)];
            if let Some(cur) = map.get(yi).and_then(|row| row.get(xi)) {
                let viable = *cur != '#' && new_trajectory.insert([xi, yi]);
                let cost_viable = *global_trajectory.get(&[xi, yi]).unwrap_or(&0) < new_trajectory.len();
                if viable && cost_viable {
                    global_trajectory.insert([xi, yi], new_trajectory.len());
                    paths.push(([xi, yi], new_trajectory));
                }
            }
        }
    }
    res
}

fn main() {
    let input = include_str!("./input.txt");
    let res = traverse_map(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(traverse_map(test_input), 94);
    }
}
