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

fn count_dropped_balls_col(col: impl Iterator<Item = char>) -> usize {
    let mut cubes = vec![Cube::new(0)];
    let mut len = 0;
    for (i, c) in col.enumerate() {
        len += 1;
        match c {
            '#' => cubes.push(Cube::new(i + 1)),
            'O' => cubes.last_mut().unwrap().ball_counter += 1,
            _ => (),
        }
    }
    cubes.into_iter()
        .map(|cube| {
            let upper_bound = len - cube.index;
            let lower_bound = upper_bound + 1 - cube.ball_counter;
            ((upper_bound + lower_bound) * cube.ball_counter) / 2
        })
        .sum()
}

fn count_dropped_balls(input: &str) -> usize {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let (max_x, max_y) = (matrix[0].len(), matrix.len());
    (0..max_x)
        .map(|x| count_dropped_balls_col((0..max_y).map(|y| matrix[y][x])))
        .sum()
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
        assert_eq!(count_dropped_balls(test_input), 136);
    }
}