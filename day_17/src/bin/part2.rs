use std::collections::{BinaryHeap, HashSet};
use std::cmp::{Reverse, Ordering};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, EnumIter, Clone, Copy, Hash)]
enum Direction {
    North,
    West,
    South,
    East
}

impl Direction {
    fn to_coords(self) -> [isize; 2] {
        match self {
            Direction::North => [0, -1],
            Direction::West => [-1, 0],
            Direction::South => [0, 1],
            Direction::East => [1, 0],
        }
    }

    fn reverse(self) -> Self {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            Direction::South => Direction::North
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HeatStep {
    x: usize,
    y: usize,
    dir: Direction,
    streak: u8,
    heat: usize,
}

impl Ord for HeatStep {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heat.cmp(&other.heat)
    }
}

impl PartialOrd for HeatStep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn path_finder(input: &str) -> usize {
    let table: Vec<Vec<usize>> = input.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect())
        .collect();
    let (x_max, y_max) = (table[0].len(), table.len());
    
    let mut heap = BinaryHeap::from([
        Reverse(HeatStep { x: 1, y: 0, dir: Direction::East, streak: 1, heat: table[0][1] }),
        Reverse(HeatStep { x: 0, y: 1, dir: Direction::South, streak: 1, heat: table[1][0] })
    ]);
    let mut visited_set: HashSet<(usize, usize, Direction, u8)> = HashSet::new();

    while let Some(Reverse(HeatStep { x, y, dir, streak, heat })) = heap.pop() {
        if (x, y) == (x_max - 1, y_max - 1) && streak >= 3 {
            return heat
        }

        for next_dir in Direction::iter().filter(|cur_dir| *cur_dir != dir.reverse()) {
            let next_streak = if next_dir == dir {
                streak + 1
            } else {
                0
            };
            let [dx, dy] = next_dir.to_coords();
            let [xn, yn] = [x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)];

            if xn < x_max && yn < y_max && next_streak < 10 && (next_dir == dir || streak >= 3) {
                let next_heat = heat + table[yn][xn];
                if visited_set.insert((xn, yn, next_dir, next_streak)) {
                    let next_step = HeatStep { x: xn, y: yn, dir: next_dir, streak: next_streak, heat: next_heat };

                    heap.push(Reverse(next_step));
                }
            }
        }
    }
    unreachable!();
}

fn main() {
    let input = include_str!("./input.txt");
    let res  = path_finder(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
43226746555331";
        assert_eq!(path_finder(test_input), 94);
    }
}