use nom::{
    character::complete::{space1, char as nom_char, u64 as nom_u64}, 
    IResult, 
    multi::{separated_list1, many1}, 
    branch::alt, sequence::preceded, combinator::recognize
};
use cached::proc_macro::cached;
use cached::SizedCache;


fn parse_record(input: &str) -> IResult<&str, (&str, Vec<u64>)> {
    let (input, springs) = recognize(many1(alt((nom_char('?'), nom_char('.'), nom_char('#')))))(input)?;
    let (input, nums) = preceded(space1, separated_list1(nom_char(','), nom_u64))(input)?;
    Ok((input, (springs, nums)))
}

#[cached(
    type = "SizedCache<String, u64>",
    create = "{ SizedCache::with_size(64) }",
    convert = r#"{ format!("{springs}{nums:?}") }"#
)]
fn count_arrangement_line(springs: &str, nums: &[u64]) -> u64 {
    let mut res = 0;
    if let Some(c) = springs.chars().next() {
        if let Some(n) = nums.iter().next() {
            let n = *n as usize;
            if matches!(c, '.' | '?') {
                res += count_arrangement_line(&springs[1..], nums);
            }
            if matches!(c, '#' | '?') && springs.len() >= n && !springs[..n].contains('.') { 
                if springs.len() == n {
                    res += count_arrangement_line(&springs[n..], &nums[1..]);
                } else if springs.chars().nth(n).unwrap() != '#' {
                    res += count_arrangement_line(&springs[n + 1..], &nums[1..]);
                } 
            }
        } else {
            return !springs.contains('#') as u64
        }
    } else  {
        return nums.is_empty() as u64
    } 
    res
}

fn count_arrangements(s: &str) -> u64 {
    s.lines().map(|line| {
        let (_, (springs, nums)) = parse_record(line).unwrap();
        let nums = [&nums[..]; 5].concat();
        let springs = [springs; 5].join("?");
        count_arrangement_line(&springs, &nums)
    }).sum()
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
        assert_eq!(count_arrangements(test_input), 525152);
    }
}