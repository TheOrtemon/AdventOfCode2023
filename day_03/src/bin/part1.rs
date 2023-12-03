use regex::Regex;

fn adjecent_matrix_gen(v: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let max_y = v.len();
    let max_x = v[0].len();
    let mut is_adjacent_num = vec![vec![false; max_x]; max_y];

    const POSSIBLE_WAYS: &[[isize; 2]] = &[
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];
    for (y, line) in v.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if !(char.is_ascii_digit() || *char == '.') {
                for [dx, dy] in POSSIBLE_WAYS {
                    let (xi, yi) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                    if (0..max_x).contains(&xi) && (0..max_y).contains(&yi) {
                        is_adjacent_num[yi][xi] = true;
                    }
                }
            }
        }
    }

    for (y, line) in v.iter().enumerate() {
        let mut prev_is_adjacent_number = false;
        for (x, char) in line.iter().enumerate().rev() {
            if char.is_ascii_digit() {
                if prev_is_adjacent_number {
                    is_adjacent_num[y][x] = true;
                } else {
                    prev_is_adjacent_number = is_adjacent_num[y][x];
                }
            } else {
                prev_is_adjacent_number = false;
            }
        }
    }
    is_adjacent_num
}

fn count_adjecent_nums(s: &str) -> u32 {
    let v = s
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let is_adjacent_num = adjecent_matrix_gen(&v);

    let re = Regex::new(r"\d+").unwrap();
    s.lines()
        .enumerate()
        .map(|(y, line)| re.find_iter(line)
            .map(|capture| {
                if is_adjacent_num[y][capture.start()] {
                    capture.as_str().parse().unwrap()
                } else {
                    0
                }
            }).sum::<u32>()
        ).sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let res = count_adjecent_nums(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(count_adjecent_nums(test_input), 4361);
    }
}
