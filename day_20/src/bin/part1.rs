use std::collections::{HashMap, VecDeque};

use nom::{
    IResult, 
    branch::alt, 
    multi::separated_list1, 
    bytes::complete::tag, 
    character::complete::alpha1, sequence::preceded 
};

#[derive(Debug, Clone, Copy)]
enum Switch {
    FlipFlop,
    Conjunction
}

fn parse_destination(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1)(input)
}

type Destination<'a> = (Switch, bool, Vec<&'a str>);

fn parse_line(input: &str) -> IResult<&str, (&str, Destination)> {
    let (input, symbol) = alt((tag("%"), tag("&")))(input)?;
    let symbol = match symbol {
        "%" => Switch::FlipFlop,
        "&" => Switch::Conjunction,
        _ => unreachable!()
    };
    let (input, label) = alpha1(input)?;
    let (input, dest) = preceded(tag(" -> "), parse_destination)(input)?;

    Ok((input, (label, (symbol, false, dest))))
}

fn module_switcher(input: &str) -> usize {
    let mut lines = input.lines();
    let mut inputs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut paths: HashMap<&str, Destination> = lines
        .clone()
        .filter(|line| !line.contains("broadcaster"))
        .map(|line| parse_line(line).unwrap().1)
        .inspect(|path| {
            path.1.2.iter().for_each(|label| {
                inputs.entry(label).or_default().push(path.0);
            });
        })
        .collect();
    let mut lows = 0;
    let mut highs = 0;

    let start_dest_str = lines.find(|line| line.contains("broadcaster"))
        .unwrap()
        .split("-> ")
        .last()
        .unwrap();
    let start_dest = parse_destination(start_dest_str).unwrap().1;
    start_dest.iter()
        .for_each(|label| inputs.entry(label)
            .or_default()
            .push("broadcaster")
        );
    let mut deq = VecDeque::new();
    for _ in 0..1000 {
        start_dest.iter().for_each(|label| deq.push_back((false, *label)));
        lows += 1;
        while let Some((high, label)) = deq.pop_front() {
            if high {
                highs += 1
            } else {
                lows += 1
            };
            let paths_ref = paths.clone();
            let Some((swtch, flag, destinations)) = paths.get_mut(label) else {continue;};
            let pulse_option = match swtch {
                Switch::FlipFlop => if !high {
                    *flag = !*flag;
                    Some(*flag)
                } else {
                    None
                },
                Switch::Conjunction => {
                    *flag = !inputs.get(label).unwrap().iter().all(|input| paths_ref.get(input).unwrap().1);
                    Some(*flag)
                },
            };
            if let Some(pulse) = pulse_option {
                for dest in destinations {
                    deq.push_back((pulse, dest));
                }
            }
        }
    }
    lows * highs
}

fn main() {
    let input = include_str!("./input.txt");
    let res  = module_switcher(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(module_switcher(test_input), 32000000);
    }
}