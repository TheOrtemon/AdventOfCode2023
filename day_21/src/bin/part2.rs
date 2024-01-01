use std::collections::BTreeSet;

fn explore_map(input: &str, step_num: usize) -> usize {
    let step_num = step_num - 1;
    let directions: [[isize; 2]; 4] = [
        [0, -1],
        [-1, 0],
        [0, 1],
        [1, 0]
    ];
    let map: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let [x_len, y_len] = [map[0].len(), map.len()];
    let start = map.iter()
        .enumerate()
        .find_map(|(y, row)| row.iter()
            .enumerate()
            .find_map(|(x, c)| if *c == 'S' {Some([x as isize, y as isize])} else {None})
        ).unwrap();
    let mut odd_set = BTreeSet::new();
    let mut even_set = BTreeSet::new();
    let mut temp_set = BTreeSet::from([start]);

    let mut step_values = vec![];
    let mut cycle_len: Option<usize> = None;
    let mut acc_koef: Option<usize> = None;

    'outer: for iteration in 1..=usize::MAX {
        let mut next_temp_set = BTreeSet::new();
        while let Some([x, y]) = temp_set.pop_first() {
            for [dx, dy] in directions.iter() {
                let [xi, yi] = [x + dx, y + dy];
                let isnt_wall = map
                    .get(yi.rem_euclid(y_len as isize) as usize)
                    .and_then(|row| row.get(xi.rem_euclid(x_len as isize) as usize))
                    .filter(|&c| *c != '#')
                    .is_some();
                if isnt_wall {
                    let next_set = if iteration % 2 == 0 {
                        &mut even_set
                    } else {
                        &mut odd_set
                    };
                    if next_set.insert([xi, yi]) {
                        next_temp_set.insert([xi, yi]);
                    };
                }
            }
        }
        let cur_steps = if iteration % 2 == 0 {
            even_set.len()
        } else {
            odd_set.len()
        };
        step_values.push(cur_steps);
        if cycle_len.is_none() && iteration > 64 {
            let lower_boundary = iteration / 4 - 1;
            let last_steps = step_values[iteration - 1];
            for i in 1..lower_boundary {
                let d = step_values[iteration - i - 1] as isize;
                let c = step_values[iteration - 2 * i - 1] as isize;
                let b = step_values[iteration - 3 * i - 1] as isize;
                let a = step_values[iteration - 4 * i - 1] as isize;

                let acc3 = c + last_steps as isize - 2 * d;
                let acc2 = b + d - 2 * c;
                let acc1 = a + c - 2 * b;

                if acc1 == acc2 && acc2 == acc3{
                    cycle_len = Some(i);
                    acc_koef = Some(acc1 as usize);
                    break 'outer;
                }
            }
        }
        temp_set = next_temp_set;
    }
    let cycle_len = cycle_len.unwrap();
    let acc_koef = acc_koef.unwrap();
    let nested_step_values = step_values.iter()
        .enumerate()
        .fold(vec![vec![]; cycle_len], |mut output, (i, cur)| {
            output[i % cycle_len].push((i, *cur));
            output
    });
    let needed_step_values = &nested_step_values[step_num % cycle_len];
    let acc_koef = acc_koef / 2;
    let [(i_prelast, val_prelast), (_, val_last)] = needed_step_values[needed_step_values.len() - 2..] else {unreachable!()};
    let speed_koef = val_last - val_prelast - acc_koef;
    let pos_koef = val_prelast;
    let res_cycles_num = (step_num - i_prelast) / cycle_len;

    pos_koef + speed_koef * res_cycles_num + acc_koef * res_cycles_num * res_cycles_num // c + bx + ax^2

}

fn main() {
    let input = include_str!("./input.txt");
    let res  = explore_map(input, 26501365);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(explore_map(test_input, 5000), 16733044);
        assert_eq!(explore_map(test_input, 500), 167004);
    }
}
