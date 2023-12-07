use std::collections::HashMap;
use nom::IResult;
use nom::character::complete::{alphanumeric1, u32 as d};
use nom::sequence::tuple;
use nom_supreme::{tag::complete::tag, ParserExt};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Hand {
    HighCard(Vec<Card>), // 5
    OnePair(Vec<Card>), // 4
    TwoPair(Vec<Card>), // 3
    ThreeIdentical(Vec<Card>), // 3
    FullHouse(Vec<Card>), // 2
    FourIdentical(Vec<Card>), // 2
    FiveIdentical(Vec<Card>), // 1
}


fn char_to_card(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => unreachable!()
        }
}

fn string_to_hand(input: &str) -> IResult<&str, (Hand, u32)> {
    let (input, (cards, bet)) = tuple((alphanumeric1, d.preceded_by(tag(" "))))(input)?;

    let hand_undefined: Vec<Card> = cards.chars().map(char_to_card).collect();
    let mut card_counter: HashMap<&Card, usize> = HashMap::new();
    for card in hand_undefined.iter() {
        *card_counter.entry(card).or_insert(0) += 1;
    }
    let hand = match card_counter.len() {
        1 => Hand::FiveIdentical(hand_undefined),
        2 => if *card_counter.values().max().unwrap() == 4 {
            Hand::FourIdentical(hand_undefined)
        } else {
            Hand::FullHouse(hand_undefined)
        },
        3 => if *card_counter.values().max().unwrap() ==  3 {
            Hand::ThreeIdentical(hand_undefined)
        } else {
            Hand::TwoPair(hand_undefined)
        },
        4 => Hand::OnePair(hand_undefined),
        5 => Hand::HighCard(hand_undefined),
        _ => unreachable!()
    };

    Ok((input, (hand, bet)))
}

fn play_poker_game(s: &str) -> u32 {
    let mut hands: Vec<(Hand, u32)> = s.lines()
        .map(|line| string_to_hand(line).unwrap().1)
        .collect();
    hands.sort();
    hands.iter().enumerate().map(|(i, (_, bet))| (i + 1) as u32 * bet).sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let res = play_poker_game(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(play_poker_game(test_input), 6440);
    }
}