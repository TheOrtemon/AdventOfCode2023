use itertools::Itertools;
use nom::{
    character::complete::{space1, char as nom_char, u64 as nom_u64}, 
    IResult, 
    multi::{separated_list1, many1}, 
    branch::alt, sequence::preceded
};

fn is_valid_spring(spring: Vec<char>, nums: &[u64]) -> bool {
    let mut actual_nums = vec![];
    let binding = spring.into_iter()
        .group_by(|&c| c);
    let res = binding .into_iter()
        .map(|(_ , group)| group.collect::<Vec<char>>());
    for group in res {
        if group[0] == '#' {
            actual_nums.push(group.len() as u64);
        }
    }
    nums == actual_nums
}

fn parse_record(input: &str) -> IResult<&str, (Vec<char>, Vec<u64>)> {
    let(input, springs) = many1(alt((nom_char('?'), nom_char('.'), nom_char('#'))))(input)?;
    let (input, nums) = preceded(space1, separated_list1(nom_char(','), nom_u64))(input)?;
    Ok((input, (springs, nums)))
}

fn count_arrangement_line(line: &str) -> u64 {
    let (_, (springs, nums)) = parse_record(line).unwrap();
    let mut res = 0;
    let unknowns: Vec<u64> = springs.iter()
        .enumerate()
        .filter_map(|(i, spring)| if *spring == '?' { Some(i as u64) } else { None })
        .collect();
    for product in (0..unknowns.len()).map(|_| ['#', '.'].iter()).multi_cartesian_product() {
        let mut possible_springs = springs.clone();
        for (i, spring) in unknowns.iter().zip(product) {
            possible_springs[*i as usize] = *spring;
        }
        if is_valid_spring(possible_springs, &nums) {
            res += 1;
        }
    }
    res
}

fn count_arrangements(s: &str) -> u64 {
    s.lines().map(count_arrangement_line).sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let res  = count_arrangements(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(count_arrangements(test_input), 21);
    }
}