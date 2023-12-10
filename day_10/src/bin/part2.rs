use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Pipe {
    Start,
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground
}

impl Pipe {
    fn from(pipe: char) -> Pipe {
        match pipe {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NE,
            'J' => Pipe::NW,      
            '7' => Pipe::SW,      
            'F' => Pipe::SE,     
            'S' => Pipe::Start,     
            '.' => Pipe::Ground,
            _ => unreachable!()
        }
    }

    fn is_pluggable(&self, other: &Pipe, x: isize, y: isize) -> bool {
        match (x, y) {
            (0, -1) => matches!(self, Pipe::Start | Pipe::Vertical | Pipe::NE | Pipe::NW) && 
                matches!(other, Pipe::Vertical | Pipe::SE | Pipe::SW),
            (0, 1) => matches!(self, Pipe::Start | Pipe::Vertical | Pipe::SE | Pipe::SW) &&
                matches!(other, Pipe::Vertical | Pipe::NE | Pipe::NW),
            (-1, 0) => matches!(self, Pipe::Start | Pipe::Horizontal | Pipe::NW | Pipe::SW) && 
                matches!(other, Pipe::Horizontal | Pipe::NE | Pipe::SE),
            (1, 0) => matches!(self, Pipe::Start | Pipe::Horizontal | Pipe::NE | Pipe::SE) &&
                matches!(other, Pipe::Horizontal | Pipe::NW | Pipe::SW),
            (_, _) => unreachable!()
        }
    }
}

fn get_pluggable_neighbours(
    pipe_table: &[Vec<Pipe>], 
    visited_table: &[Vec<bool>], 
    cur_point: [usize; 2], 
) -> Vec<[usize; 2]> {
    let directions: [[isize; 2]; 4] = [
        [-1, 0],
        [1, 0],
        [0, -1],
        [0, 1]
    ];
    let [cur_x , cur_y] = cur_point;
    directions.iter()
        .filter_map(|[dx, dy]| {
            let cur = &pipe_table[cur_y][cur_x];
            let (neighbour_x, neighbour_y) = (
                cur_x.wrapping_add_signed(*dx), 
                cur_y.wrapping_add_signed(*dy)
            );
            if !(0..pipe_table.len()).contains(&neighbour_y) || !(0..pipe_table[0].len()).contains(&neighbour_x) {
                return None
            }
            let neighbour = &pipe_table[neighbour_y][neighbour_x];
            if cur.is_pluggable(neighbour, *dx, *dy) && !visited_table[neighbour_y][neighbour_x] {
                Some([neighbour_x, neighbour_y])
            } else {
                None
            }
        }).collect::<Vec<[usize; 2]>>()
}

fn extrapolate(s: &str) -> usize {
    let pipe_table: Vec<Vec<Pipe>> = s.lines()
        .map(|line| line.chars().map(Pipe::from).collect())
        .collect();
    let (x_len, y_len) = (pipe_table[0].len(), pipe_table.len());
    let mut visited_table: Vec<Vec<bool>> = vec![vec![false; x_len]; y_len];
    let start: [usize; 2] = pipe_table.iter()
        .enumerate()
        .find_map(|(y, row)| row.iter()
            .position(|pipe| matches!(pipe, Pipe::Start))
            .map(|x| [x, y]))
        .unwrap();
    let [start_x, start_y] = start;
    visited_table[start_y][start_x] = true;
    let mut paths = [start; 2];
    'outer: loop {
        let neighbour_a = get_pluggable_neighbours(&pipe_table, &visited_table, paths[0]);
        let neighbour_b = get_pluggable_neighbours(&pipe_table, &visited_table, paths[1]);
        let neighbours = [neighbour_a.into_iter().next(), neighbour_b.into_iter().last()];
        for (path, neighbour) in paths.iter_mut().zip(neighbours) {
            if let Some(neighbour_point) = neighbour {
                let [x, y] = neighbour_point;
                visited_table[y][x] = true;
                *path = neighbour_point;
            } else {
                break 'outer
            }
        }
    }

    let mut pipe_table = pipe_table;
    let directions: [[isize; 2]; 4] = [
        [-1, 0],
        [1, 0],
        [0, -1],
        [0, 1]
    ];
    
    let pipes = [Pipe::Vertical, Pipe::Horizontal, Pipe::NE, Pipe::NW, Pipe::SE, Pipe::SW];
    let mut poss = HashSet::from(pipes.clone());
    for [dx, dy] in directions.iter() {
        let (neighbour_x, neighbour_y) = (
            start_x.wrapping_add_signed(*dx), 
            start_y.wrapping_add_signed(*dy)
        );
        if !(0..pipe_table.len()).contains(&neighbour_y) || !(0..pipe_table[0].len()).contains(&neighbour_x) {
            continue;
        }
        let neighbour = &pipe_table[neighbour_y][neighbour_x];
        let mut cur_poss = HashSet::new();
        for cur in pipes.iter() {
            if cur.is_pluggable(neighbour, *dx, *dy) && visited_table[neighbour_y][neighbour_x] {
                cur_poss.insert(cur);
            } 
        }
        if !cur_poss.is_empty() {
            poss.retain(|x| cur_poss.contains(x));
        } 
    };

    
    let new_start = poss.into_iter().next().unwrap();

    pipe_table[start_y][start_x] = new_start;
    for (y, line) in visited_table.iter_mut().enumerate() {
        let mut walls: usize = 0;
        let mut last_corner: Option<Pipe> = None;
        for (x, visited) in line.iter_mut().enumerate() {
            let cur_point = &pipe_table[y][x];
            if !*visited && walls % 2 == 0 {
                *visited = true;
            } else if *visited {
                match cur_point {
                    Pipe::Vertical => {
                        walls += 1;
                        last_corner = None;
                    },
                    Pipe::NE => {
                        walls += 1;
                        last_corner = Some(Pipe::NE);
                    }
                    Pipe::SE => {
                        walls += 1;
                        last_corner = Some(Pipe::SE);
                    },
                    Pipe::NW => {
                        if !matches!(last_corner, Some(Pipe::SE)) {
                            walls += 1;
                        }
                        last_corner = Some(Pipe::NW);
                    },
                    Pipe::SW => {
                        if !matches!(last_corner, Some(Pipe::NE)) {
                            walls += 1;
                        }
                        last_corner = Some(Pipe::SW);
                    },
                    _ => (),
                }
            } else {
                last_corner = None;
            }
        }
    }

    visited_table.iter().flat_map(|line| line.iter().map(|flag| !*flag as usize)).sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let res = extrapolate(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(extrapolate(test_input), 10);
    }
}