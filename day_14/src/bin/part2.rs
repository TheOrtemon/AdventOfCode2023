use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Cube {
    index: usize,
    ball_counter: usize
}
impl Cube {
    fn new(index: usize) -> Cube {
        Cube { index, ball_counter: 0 }
    }
}

fn get_load(matrix: &[Vec<char>]) -> usize {
    let y_len = matrix.len();
    (0..matrix[0].len())
        .map(|x| (0..y_len).map(|y| {
            if matches!(matrix[y][x], 'O') {
                y_len - y
            } else {
                0
            }
        }).sum::<usize>()
    ).sum::<usize>()
}

fn count_dropped_balls_col(col: impl Iterator<Item = char>) -> Vec<Cube> {
    let mut cubes = vec![Cube::new(0)];
    // let mut len = 0;
    for (i, c) in col.enumerate() {
        // len += 1;
        match c {
            '#' => cubes.push(Cube::new(i + 1)),
            'O' => cubes.last_mut().unwrap().ball_counter += 1,
            _ => (),
        }
    }
    cubes
}

fn count_dropped_balls(input: &str) -> usize {
    let mut matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let (max_x, max_y) = (matrix[0].len(), matrix.len());

    let mut prev_results: HashMap<u64, [usize; 2]> = HashMap::new();
    for iteration in 0..1_000_000_000 {
        let cube_cols = (0..max_x)
            .map(|x| count_dropped_balls_col((0..max_y).map(|y| matrix[y][x])))
            .collect::<Vec<_>>();
        for (x, cube_col) in cube_cols.into_iter().enumerate() {
            for cube in cube_col {
                for entry in matrix.iter_mut().skip(cube.index).take(cube.ball_counter) {
                    entry[x] = 'O';
                }
                if (cube.index + cube.ball_counter) >= max_y {
                    continue;
                }
                for entry in matrix.iter_mut().take(max_y).skip(cube.index + cube.ball_counter) {
                    if entry[x] == '#' {
                        break;
                    } else {
                        entry[x] = '.';
                    }
                }
            }
        }

        let cube_lines = matrix.clone()
            .into_iter().map(|line| count_dropped_balls_col(line.into_iter()))
            .collect::<Vec<_>>();
        for (y, line) in cube_lines.into_iter().enumerate() {
            for cube in line {
                for entry in matrix[y].iter_mut().skip(cube.index).take(cube.ball_counter) {
                    *entry = 'O';
                }
                if (cube.index + cube.ball_counter) >= max_x {
                    continue;
                }
                for entry in matrix[y].iter_mut().skip(cube.index + cube.ball_counter) {
                    if *entry == '#' {
                        break;
                    } else {
                        *entry = '.';
                    }
                }
            }
        }

        let cube_rev_cols = (0..max_x)
            .map(|x| count_dropped_balls_col((0..max_y).rev().map(|y| matrix[y][x])))
            .collect::<Vec<_>>();
        for (x, cube_col) in cube_rev_cols.into_iter().enumerate() {
            for cube in cube_col {
                for entry in matrix.iter_mut().rev().skip(cube.index).take(cube.ball_counter) {
                    entry[x] = 'O';
                }
                if (cube.index + cube.ball_counter) >= max_y {
                    continue;
                }
                for entry in matrix.iter_mut().rev().skip(cube.index + cube.ball_counter) {
                    if entry[x] == '#' {
                        break;
                    } else {
                        entry[x] = '.';
                    }
                }
            }
        }    
        let cube_rev_lines = matrix.clone()
            .into_iter()
            .map(|line| count_dropped_balls_col(line.into_iter().rev()))
            .collect::<Vec<_>>();
        for (y, line) in cube_rev_lines.into_iter().enumerate() {
            for cube in line {
                for entry in matrix[y].iter_mut().rev().skip(cube.index).take(cube.ball_counter) {
                    *entry = 'O';
                }
                if (cube.index + cube.ball_counter) >= max_x {
                    continue;
                }
                for entry in matrix[y].iter_mut().rev().skip(cube.index + cube.ball_counter) {
                    if *entry == '#' {
                        break;
                    } else {
                        *entry = '.';
                    }
                }
            }
        }
        let load = get_load(&matrix);
        let mut hasher = DefaultHasher::new();
        matrix.hash(&mut hasher);
        let hash = hasher.finish();
        if let Some([prev_iteration, _]) = prev_results.get(&hash) {
            let cycled = 1_000_000_000 - prev_iteration - 1;
            let modulo = cycled % (iteration - prev_iteration);
            let sync_iteration = prev_iteration + modulo;
            return prev_results.into_values()
                .filter_map(|[i, load]| if i == sync_iteration { Some(load) } else { None })
                .next()
                .unwrap()
        }
        prev_results.insert(hash, [iteration, load]);
    }
    unreachable!();
}
fn main() {
    let input = include_str!("./input.txt");
    let res  = count_dropped_balls(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(count_dropped_balls(test_input), 64);
    }
}