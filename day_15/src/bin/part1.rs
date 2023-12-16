fn get_hash(slice: &str) -> usize {
    slice.chars()
        .map(|c| c as usize)
        .fold(0, |acc, cur| ((acc + cur) * 17 ) % 256 )
}
fn sum_hashes(input: &str) -> usize {
    input.split(',')
        .map(|slice| get_hash(slice.trim()))
        .sum()

}

fn main() {
    let input = include_str!("./input.txt");
    let res  = sum_hashes(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(sum_hashes(test_input), 1320);
    }
}