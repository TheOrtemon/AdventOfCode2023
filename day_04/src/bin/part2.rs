use std::collections::HashSet;

fn count_winning_points(line: &str) -> usize {
    let line = line.split(": ").last().unwrap();
    let mut parts = line.split('|');
    let (winning, mine) = (parts.next().unwrap(), parts.last().unwrap());
    let winning_set: HashSet<u32> = winning.split_whitespace().map(|num| num.parse().unwrap()).collect();
    let mine_iter = mine.split_whitespace().map(|num| num.parse::<u32>().unwrap());
    mine_iter
        .filter(|num| winning_set.contains(num))
        .count()
}

fn process_lottery_ticket(ticket: &str) -> usize {
    let lines = ticket.lines();
    let n = lines.clone().count();
    let mut points = vec![1; n];
    let mut res: usize = 0;
    for (i, line) in lines.enumerate() {
        let point = points[i];
        res += point;
        let round = count_winning_points(line);
        for j in 0_usize..round {
            let index = i + j + 1;
            if index < n {
                points[i + j + 1] += point;
            } else {
                break
            }
        }
    }
    res
}


fn main() {
    let input = include_str!("./input.txt");
    let res = process_lottery_ticket(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process_lottery_ticket(test_input), 30);
    }
}