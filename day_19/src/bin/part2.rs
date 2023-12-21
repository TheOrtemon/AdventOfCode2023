use nom::{
    IResult, 
    sequence::{delimited, preceded}, 
    bytes::streaming::tag, 
    character::complete::{alpha1, self}, 
    multi::separated_list1, branch::alt
};
use std::{ops::Range, cmp::Ordering, collections::HashMap};

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


#[derive(Debug, Clone, PartialEq, Eq)]
struct Part {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>
}

#[derive(Debug)]
struct Condition<'a> {
    letter: Letter,
    cmp: Ordering,
    value: usize,
    dest: Destination<'a>
}

#[derive(Debug)]
enum Step<'a> {
    Condition(Condition<'a>),
    Destination(Destination<'a>)
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

    Ok((input, Condition { letter, cmp, value: value as usize, dest }))
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

fn custom_sorter(input: &str) -> usize {
    let workflows = input.split("\n\n")
        .flat_map(|block| block.split("\r\n\r\n"))
        .next()
        .unwrap()
        .split('\n')
        .map(|line| parse_workflow(line).unwrap().1);
    let workflow_map: HashMap<&str, Vec<Step>> = HashMap::from_iter(workflows);

    let mut wf_vec = vec![("in", Part { x: 1..4001, m: 1..4001, a: 1..4001, s: 1..4001 })];
    let mut res = 0;
    while let Some((label, part)) = wf_vec.pop() {
        let mut part = part;
        let cur_steps = workflow_map.get(label).unwrap();
        for step in cur_steps {
            match step {
                Step::Destination(dest) => match dest {
                    Destination::Accepted(flag) => if *flag {
                        res += [&part.x, &part.m, &part.a, &part.s]
                            .iter()
                            .map(|n| n.len())
                            .product::<usize>();
                    },
                    Destination::Label(label) => wf_vec.push((label, part.clone())),
                },
                Step::Condition(cond) => {
                    let letter_range = match cond.letter {
                        Letter::X => &part.x,
                        Letter::M => &part.m,
                        Letter::A => &part.a,
                        Letter::S => &part.s,
                    };
                    if let Some((range_true, range_false)) = match cond.cmp {
                        Ordering::Less => {
                            if letter_range.start < cond.value {
                                Some((letter_range.start..cond.value, cond.value..letter_range.end))
                            } else {
                                None
                            }
                        },
                        Ordering::Greater => {
                            if letter_range.end > cond.value + 1 {
                                Some((cond.value + 1..letter_range.end, letter_range.start..cond.value + 1))
                            } else {
                                None
                            }
                        }
                        Ordering::Equal => unreachable!()
                    } {
                        let mut part_true = part.clone();
                        *match cond.letter {
                            Letter::X => &mut part_true.x,
                            Letter::M => &mut part_true.m,
                            Letter::A => &mut part_true.a,
                            Letter::S => &mut part_true.s,
                        } = range_true;
                        
                        match cond.dest {
                            Destination::Accepted(flag) => if flag {
                                res += [&part_true.x, &part_true.m, &part_true.a, &part_true.s]
                                    .iter()
                                    .map(|n| n.len())
                                    .product::<usize>();
                            },
                            Destination::Label(label) => wf_vec.push((label, part_true)),
                        }
                        *match cond.letter {
                            Letter::X => &mut part.x,
                            Letter::M => &mut part.m,
                            Letter::A => &mut part.a,
                            Letter::S => &mut part.s,
                        } = range_false;
                    }
                }
            }
        }
    }
    res
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
        assert_eq!(custom_sorter(test_input), 167409079868000);
    }
}