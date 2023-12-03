use regex::Regex;
use std::collections::HashMap;

fn gear_points_gen(s: &str) -> Vec<[[usize; 2]; 2]> {
    let v = s
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let max_y = v.len();
    let max_x = v[0].len();

    let mut coords: Vec<[[usize; 2]; 2]> = vec![];

    const POSSIBLE_WAYS_MIDDLE: &[[isize; 2]] = &[
        [1, 0],
        [-1, 0],
    ];
    const POSSIBLE_WAYS_UPPER: &[[isize; 2]] = &[
        [1, -1],
        [0, -1],
        [-1, -1],
    ];
    const POSSIBLE_WAYS_LOWER: &[[isize; 2]] = &[
        [1, 1],
        [0, 1],
        [-1, 1],
    ];

    for (y, line) in v.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == '*' {
                let mut cur_points = [[usize::MAX; 2]; 2];
                let mut counter = 0;
                let mut the_ways = [
                    (POSSIBLE_WAYS_MIDDLE, true, true),
                    (POSSIBLE_WAYS_UPPER, false, false),
                    (POSSIBLE_WAYS_LOWER, false, false),
                ];
                for (dx_dy_list, is_prev_digit, is_middle_case) in the_ways.iter_mut() {
                    for [dx, dy] in dx_dy_list.iter() {
                        let (xi, yi) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                        if (0..max_x).contains(&xi) && (0..max_y).contains(&yi) && v[yi][xi].is_ascii_digit() {
                            if !*is_prev_digit || *is_middle_case {
                                counter += 1;
                                if counter >= 3 {
                                    break;
                                }
                            }
                            cur_points[counter - 1] = [xi, yi];
                            *is_prev_digit = true;
                        } else {
                            *is_prev_digit = false;
                        }
                    }
                }

                if counter == 2 {
                    for point in cur_points.iter_mut() {
                        while point[0] > 0 && v[point[1]][point[0] - 1].is_ascii_digit() {
                            point[0] -= 1;
                        }

                    }
                    coords.push(cur_points);
                }
            }
        }
    }
    coords
}

fn num_map_gen(s: &str) -> HashMap<[usize; 2], u32>{
    let re = Regex::new(r"\d+").unwrap();
    s.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            re.find_iter(line).map(move |capture| {
                let x = capture.start();
                let value: u32 = capture.as_str().parse().unwrap();
                ([x, y], value)
            })
        })
        .collect()
}

fn count_adjecent_nums(s: &str) -> u32 {
    let coords = gear_points_gen(s);

    let points = num_map_gen(s);

    coords.into_iter()
        .map(|[a, b]| points.get(&a).unwrap() * points.get(&b).unwrap())
        .sum()
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
        assert_eq!(count_adjecent_nums(test_input), 467835);
    }
    #[test]
    fn it_works2() {
        let test_input = "467*114...664.598..";
        assert_eq!(count_adjecent_nums(test_input), 53238);
    }
}
