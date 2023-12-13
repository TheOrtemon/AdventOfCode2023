use itertools::Itertools;

fn _count_reflections_one_way(matrix: &[Vec<char>], len: usize) -> usize {
    let mut res = 0;
    'outer: for ((i, first), (j, second)) in matrix.iter().enumerate().tuple_windows() {
        if first == second {
            for counter in 1..len {
                if j + counter < len && i >= counter {
                    if matrix[i - counter] != matrix[j + counter] {
                        continue 'outer;
                    } 
                } else {
                    res += j;
                    break 'outer;
                }
            }
        }
    }
    res
}

fn _count_reflections(input: &str) -> usize {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (x_len, y_len) = (matrix[0].len(), matrix.len());

    let res_y = _count_reflections_one_way(&matrix, y_len) * 100;

    let transposed_matrix = (0..x_len).map(|x| {
        let matrix = &matrix;
        (0..y_len).map(move |y| matrix[y][x]).collect::<Vec<char>>()
    }).collect_vec();
    let res_x = _count_reflections_one_way(&transposed_matrix, x_len);

    res_x + res_y
}

fn count_reflections(input: &str) -> usize {
    input
        .split("\n\n")
        .flat_map(|block| block.split("\r\n\r\n"))
        .map(_count_reflections)
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let res  = count_reflections(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(count_reflections(test_input), 405);
    }
}