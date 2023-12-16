use nom::{IResult, character::complete::{alpha1, self}, sequence::{pair, separated_pair}, branch::alt};

#[derive(Debug)]
enum Operation<'a> {
    Minus(&'a str),
    Equal(&'a str, u8)
}

fn parse_minus_operation(input: &str) -> IResult<&str, Operation> {
    let (input, (label, _)) = pair(alpha1, complete::char('-'))(input)?;
    Ok((input, Operation::Minus(label)))
}

fn parse_equal_operation(input: &str) -> IResult<&str, Operation> {
    let (input, (label, lens)) = separated_pair(alpha1, complete::char('='), complete::u8)(input)?;
    Ok((input, Operation::Equal(label, lens)))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((parse_minus_operation, parse_equal_operation))(input)
}

fn get_hash(slice: &str) -> usize {
    slice.chars()
        .map(|c| c as usize)
        .fold(0, |acc, cur| ((acc + cur) * 17 ) % 256 )
}
fn sum_lenses(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, u8)>> = vec![vec![]; 256];
    let init_seq = input.split(',')
        .map(|slice| {
            let (_, operation) = parse_operation(slice.trim()).unwrap();
            let label = match operation {
                Operation::Equal(lab, _) => lab,
                Operation::Minus(lab) => lab
            };
            (operation, get_hash(label))
        }).collect::<Vec<_>>();
    for (operation, hash) in init_seq.into_iter() {
        let cur_box = boxes.get_mut(hash).unwrap();
        match operation {
            Operation::Minus(label) => {
                let index_opt = cur_box.iter().position(|(cur_label, _)| *cur_label == label);
                if let Some(index) = index_opt {
                    cur_box.remove(index);
                }
            },
            Operation::Equal(label, lens) => {
                let index_opt = cur_box.iter().position(|(cur_label, _)| *cur_label == label);
                if let Some(index) = index_opt {
                    cur_box[index] = (label, lens);
                } else {
                    cur_box.push((label, lens));
                }
            },
        }
    }
    boxes.iter()
        .enumerate()
        .flat_map(|(j, cur_box)| cur_box.iter()
            .enumerate()
            .map(move |(i, (_, lens))| (j + 1) * (i + 1) * *lens as usize))
            .sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let res  = sum_lenses(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(sum_lenses(test_input), 145);
    }
}