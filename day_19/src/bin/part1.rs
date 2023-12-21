use nom::{
    IResult, 
    sequence::{delimited, tuple, preceded}, 
    bytes::streaming::tag, 
    character::complete::{alpha1, self}, 
    multi::separated_list1, branch::alt
};
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
enum Destination<'a> {
    Accepted(bool),
    Label(&'a str)
}

#[derive(Debug)]
enum Letter {
    X,
    M,
    A,
    S
}


#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64
}

#[derive(Debug)]
struct Condition<'a> {
    letter: Letter,
    cmp: Ordering,
    value: u64,
    dest: Destination<'a>
}

#[derive(Debug)]
enum Step<'a> {
    Condition(Condition<'a>),
    Destination(Destination<'a>)
}


fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, (x, m, a, s)) = delimited(
        tag("{x="), 
        tuple((
            complete::u64, 
            preceded(tag(",m="), complete::u64), 
            preceded(tag(",a="), complete::u64), 
            preceded(tag(",s="), complete::u64)
        )),
        tag("}")
    )(input)?;

    Ok((input, Part { x, m, a, s }))
}

fn parse_destination(input: &str) -> IResult<&str, Destination> {
    let (input, dest) = alpha1(input)?;
    let dest = match dest {
        "A" => Destination::Accepted(true),
        "R" => Destination::Accepted(false),
        _ => Destination::Label(dest)
    };

    Ok((input, dest))
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let (input, letter) = alt((tag("x"), tag("m"), tag("a"), tag("s")))(input)?;
    let letter = match letter {
        "x" => Letter::X,
        "m" => Letter::M,
        "a" => Letter::A,
        "s" => Letter::S,
        _ => unreachable!()
    };
    let (input, cmp) = alt((tag("<"), tag(">")))(input)?;
    let cmp = match cmp {
        "<" => Ordering::Less,
        ">" => Ordering::Greater,
        _ => unreachable!()
    };
    let (input, value) = complete::u64(input)?;
    let (input, dest) = preceded(tag(":"), parse_destination)(input)?;

    Ok((input, Condition { letter, cmp, value, dest }))
}

fn parse_step_condition(input: &str) -> IResult<&str, Step> {
    let (input, cond) = parse_condition(input)?;

    Ok((input, Step::Condition(cond)))
}

fn parse_step_destination(input: &str) -> IResult<&str, Step> {
    let (input, dest) = parse_destination(input)?;

    Ok((input, Step::Destination(dest)))
}

fn parse_step(input: &str) -> IResult<&str, Step> {
    let (input, step) = alt((parse_step_condition, parse_step_destination))(input)?;

    Ok((input, step))
}

fn parse_workflow(input: &str) -> IResult<&str, (&str, Vec<Step>)> {
    let (input, in_label) = alpha1(input)?;
    let (input, steps) = delimited(tag("{"), separated_list1(tag(","), parse_step), tag("}"))(input)?;

    Ok((input, (in_label, steps)))
}

fn walk_the_part(part: &Part, workflow_map: &HashMap<&str, Vec<Step>>) -> bool {
    let mut start = "in";
    loop {
        let cur_steps = workflow_map.get(start).unwrap();
        for step in cur_steps {
            match step {
                Step::Condition(cond) => {
                    let letter_val = match cond.letter {
                        Letter::X => &part.x,
                        Letter::M => &part.m,
                        Letter::A => &part.a,
                        Letter::S => &part.s,
                    };
                    if letter_val.cmp(&cond.value) == cond.cmp {
                        match cond.dest {
                            Destination::Accepted(flag) => return flag,
                            Destination::Label(label) => {
                                start = label;
                                break;
                            },
                        }
                    }
                },
                Step::Destination(dest) => match dest {
                    Destination::Accepted(flag) => return *flag,
                    Destination::Label(label) => {
                        start = label;
                        break;
                    },
                },
            }
        }
    }
}

fn custom_sorter(input: &str) -> u64 {
    let mut input = input.split("\n\n").flat_map(|block| block.split("\r\n\r\n"));
    let workflows = input.next()
        .unwrap()
        .split('\n')
        .map(|line| parse_workflow(line).unwrap().1);
    let parts = input.last()
        .unwrap()
        .split('\n')
        .map(|line| parse_part(line).unwrap().1);
    let workflow_map: HashMap<&str, Vec<Step>> = HashMap::from_iter(workflows);

    parts.filter_map(|part| {
        if walk_the_part(&part, &workflow_map) {
            Some([part.x, part.m, part.a, part.s].iter().sum::<u64>())
        } else {
            None
        }
    }).sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let res  = custom_sorter(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(custom_sorter(test_input), 19114);
    }
}