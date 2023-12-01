fn summator(input: &str) -> u32 {
    input.lines().map(|line| {
        let mut digits = line.chars().filter_map(|char| char.to_digit(10));
        let (n1, n2) = (digits.clone().next().unwrap(), digits.next_back().unwrap());
        n1 * 10 + n2
    }).sum()
}
fn main() {
    let input = include_str!("./input.txt");
    let res = summator(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(summator(test_input), 142);
    }
}