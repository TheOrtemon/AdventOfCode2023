use nom::IResult;
use nom::character::complete::{space1, i64 as d};
use nom::sequence::tuple;
use std::ops::Range;


fn parse_numbers(input: &str) -> IResult<&str, (i64, i64, i64)> {
    let (input, (a, _, b, _, c)) = tuple((d, space1, d, space1, d))(input)?;

    Ok((input, (a, b, c)))
}

fn process_map_block(block: &str) -> Vec<(Range<i64>, i64)> {
    let mut lines = block.lines();
    let _map_name = lines.next().unwrap();
    lines.map(|line| {
        let (_, (destination_start, source_start, range_len)) = parse_numbers(line).unwrap();
        let diff = destination_start - source_start;
        let range = source_start..(source_start + range_len);
        (range, diff)
    }).collect()
}
fn find_closest_location(s: &str) -> i64 {
    let mut line_blocks = s.split("\n\n");
    let seeds_str = line_blocks.next().unwrap().trim().split(": ").last().unwrap();
    let mut seeds: Vec<i64> = seeds_str.split_whitespace().map(|seed| seed.parse().unwrap()).collect();
    let maps = line_blocks.map(process_map_block).collect::<Vec<_>>();
    *seeds.iter_mut().map(|seed| {
        for map in maps.iter() {
            for (range, diff) in map.iter() {
                if range.contains(seed) {
                    *seed += diff;
                    break;
                }
            }
        }
        seed
    }).min().unwrap()
}

fn main() {
    let input = include_str!("./input.txt");
    let res = find_closest_location(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"seeds: 79 14 55 13

seed-to-soil map:
50 98 3
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(find_closest_location(test_input), 35);
    }
}