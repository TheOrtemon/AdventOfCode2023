use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
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

    fn to_bit(self) -> u8 {
        1 << (self as usize)
    }
}

#[derive(Debug)]
enum Mirror {
    Vertical,
    Horizontal,
    Slash,
    Backslash,
    Ground
}

impl Mirror {
    fn from(mirror: char) -> Mirror {
        match mirror {
            '|' => Mirror::Vertical,
            '-' => Mirror::Horizontal,
            '/' => Mirror::Slash,
            '\\' => Mirror::Backslash,
            '.' => Mirror::Ground,
            _ => unreachable!()
        }
    }

    fn direct_light(&self, dir: Direction) -> (Direction, Option<Direction>) {
        match self {
            Mirror::Vertical => match dir {
                Direction::North | Direction::South => (dir, None),
                Direction::West | Direction::East => (Direction::North, Some(Direction::South))
            },
            Mirror::Horizontal => match dir {
                Direction::North | Direction::South => (Direction::West, Some(Direction::East)),
                Direction::West | Direction::East => (dir, None)
            },
            Mirror::Slash => match dir {
                Direction::North => (Direction::East, None),
                Direction::West => (Direction::South, None),
                Direction::South => (Direction::West, None),
                Direction::East => (Direction::North, None)
            },
            Mirror::Backslash => match dir {
                Direction::North => (Direction::West, None),
                Direction::West => (Direction::North, None),
                Direction::South => (Direction::East, None),
                Direction::East => (Direction::South, None),
            },
            Mirror::Ground => (dir, None),
        }
    }
}

fn walk_the_deq(
    deq: &mut VecDeque<([usize; 2], Direction)>,
    visited_matrix: &mut [Vec<u8>],
    matrix: &[Vec<Mirror>],
    x: usize,
    y: usize,
    dir: Direction
) {
    let [dx, dy] = dir.to_coords();
    let (xi, yi) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
    let bit = dir.to_bit();

    if xi < matrix[0].len() && yi < matrix.len() && visited_matrix[yi][xi] & bit == 0 {
        visited_matrix[yi][xi] |= bit;
        deq.push_back(([xi, yi], dir));
    }
}

fn follow_the_light(
    matrix: &[Vec<Mirror>], 
    start_x: usize, 
    start_y: usize, 
    start_dir: Direction
) -> usize {
    let (max_x, max_y) = (matrix[0].len(), matrix.len());
    let mut visited_matrix: Vec<Vec<u8>> = vec![vec![0; max_x]; max_y];
    visited_matrix[start_y][start_x] |= start_dir.to_bit();
    let mut deq: VecDeque<([usize; 2], Direction)> = VecDeque::from([([start_x, start_y], start_dir)]);
    while let Some(([x, y], dir)) = deq.pop_front() {
        let cur_mirror = &matrix[y][x];
        let (next_dir_1, opt_dir) = cur_mirror.direct_light(dir);
        walk_the_deq(&mut deq, &mut visited_matrix, matrix, x, y, next_dir_1);
        if let Some(next_dir_2) = opt_dir {
            walk_the_deq(&mut deq, &mut visited_matrix, matrix, x, y, next_dir_2);
        }
    }
    visited_matrix.into_iter()
        .flat_map(|line| line.into_iter()
            .map(|visited| (visited > 0) as usize))
        .sum()
}

fn count_light_paths(input: &str) -> usize {
    let matrix: Vec<Vec<Mirror>> = input
        .lines()
        .map(|line| line
            .chars()
            .map(Mirror::from)
            .collect()
        ).collect();
    
    let (max_x, max_y) = (matrix[0].len(), matrix.len());
    (0..max_y).map(|y| (0, y, Direction::East)).chain(
        (0..max_y).map(|y| (max_x - 1, y, Direction::West))
    ).chain(
        (0..max_x).map(|x| (x, 0, Direction::South))
    ).chain(
        (0..max_x).map(|x| (x, max_y - 1, Direction::North))
    ).map(|(x, y, dir)| follow_the_light(&matrix, x, y, dir))
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("./input.txt");
    let res  = count_light_paths(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(count_light_paths(test_input), 51);
    }
}