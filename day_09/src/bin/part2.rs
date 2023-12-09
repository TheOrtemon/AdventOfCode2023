use itertools::Itertools;
use std::collections::VecDeque;


fn extrapolate_line(line: &str) -> i32 {
    let ints = line.split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<VecDeque<i32>>();
    let mut extrapolations = vec![ints];
    while !extrapolations.last().unwrap().iter().all(|num| *num == 0) {
        let iteration = extrapolations.last()
            .unwrap()
            .iter()
            .tuple_windows()
            .map(|(prev, next)| next - prev)
            .collect::<VecDeque<i32>>();
        extrapolations.push(iteration);
    }

    let extr_len = extrapolations.len();
    for idx in (0..extr_len - 1).rev() {
        let former_first = extrapolations[idx + 1][0];
        let latter = &mut extrapolations[idx];
        latter.push_front(latter[0] - former_first);
    }

    extrapolations[0][0]
}

fn extrapolate(s: &str) -> i32 {
    s.lines()
        .map(extrapolate_line)
        .sum()
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
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(extrapolate(test_input), 2);
    }
}
