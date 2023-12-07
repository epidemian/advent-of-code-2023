use itertools::Itertools;
use std::cmp::Ordering;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let mut hands: Vec<_> = input.lines().map(parse_line).try_collect()?;
    hands.sort_by_key(|(hand, _bid)| *hand);
    let ans_1: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| *bid * (i + 1) as u64)
        .sum();
    println!("{ans_1}");
    Ok(())
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct Hand {
    cards: [char; 5],
}

#[derive(Ord, Eq, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn type_(&self) -> HandType {
        let mut counts: Vec<_> = self.cards.into_iter().counts().into_values().collect();
        counts.sort();
        match counts[..] {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.type_()
            .cmp(&other.type_())
            .then_with(|| self.cards.map(card_value).cmp(&other.cards.map(card_value)))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn card_value(card_label: char) -> usize {
    [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ]
    .into_iter()
    .position(|l| l == card_label)
    .unwrap_or(0)
}

fn parse_line(line: &str) -> aoc::Result<(Hand, u64)> {
    let (hand, bid) = line.split_once(' ').ok_or("invalid input")?;
    let cards: Vec<_> = hand.chars().collect();
    let hand = Hand {
        cards: cards[..].try_into()?,
    };
    Ok((hand, bid.parse()?))
}
