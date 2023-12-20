use itertools::Itertools;
use nom::{
    IResult, 
    character::complete::{anychar, self}, 
    sequence::preceded
};

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right
}

impl Direction {
    fn from(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'L' => Direction::Left,
            'D' => Direction::Down,
            'R' => Direction::Right,
            _ => unreachable!()
        }
    }

    fn to_coords(&self) -> [isize; 2] {
        match self {
            Direction::Up => [0, -1],
            Direction::Left => [-1, 0],
            Direction::Down => [0, 1],
            Direction::Right => [1, 0],
        }
    }
}

fn parse_steps(input: &str) -> IResult<&str, (Direction, usize)> {
    let (input, dir_char) = anychar(input)?;
    let (input, num) = preceded(complete::char(' '), complete::u32)(input)?;

    Ok((input, (Direction::from(dir_char), num as usize)))
}

fn path_finder(input: &str) -> isize {
    let steps: Vec<(Direction, usize)> = input
        .lines() 
        .map(|line| parse_steps(line).unwrap().1)
        .collect();

    let mut points: Vec<[isize; 2]> = vec![[0, 0]];
    let mut max_coords = [isize::MIN; 2];
    let mut min_coords = [isize::MAX; 2];
    for (dir, len) in steps.iter() {
        let mut coords: [isize; 2] = dir.to_coords();
        let last_point = points.last().unwrap();
        let special_iterator = coords.iter_mut()
            .zip(last_point.iter())
            .zip(min_coords.iter_mut())
            .zip(max_coords.iter_mut());
        for (((dl, last_coord), min), max) in special_iterator {
            *dl = (*dl * *len as isize) + *last_coord;
            *min = *min.min(dl);
            *max = *max.max(dl);
        }
        points.push(coords);
    }
    let points: Vec<[isize; 2]> = points.into_iter()
        .map(|[x, y]| [(x - min_coords[0]), (y - min_coords[1])])
        .collect();
    let mut area = 0;
    let mut bound = 0;
    for ([prev_x, prev_y], [next_x, next_y]) in points.iter().tuple_windows() {
        bound += (next_x - prev_x + next_y - prev_y).abs();
        if prev_y == next_y {
            area += prev_y * (next_x - prev_x);
        }
    }
    let interior = area.abs() - bound / 2 + 1;
    interior + bound
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
"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(path_finder(test_input), 62);
    }
}