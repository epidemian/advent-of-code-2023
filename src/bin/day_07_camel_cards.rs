use anyhow::{bail, Context};
use itertools::Itertools;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;

    let mut hands: Vec<_> = input.lines().map(|l| parse_line(l, false)).try_collect()?;
    hands.sort_by_key(|(hand, _bid)| *hand);
    let ans_1 = total_winnings(&hands);

    let mut hands: Vec<_> = input.lines().map(|l| parse_line(l, true)).try_collect()?;
    hands.sort_by_key(|(hand, _bid)| *hand);
    let ans_2 = total_winnings(&hands);

    println!("{ans_1} {ans_2}");
    Ok(())
}

fn total_winnings(hands: &[(Hand, u64)]) -> u64 {
    hands.iter().zip(1..).map(|(&(_, bid), i)| bid * i).sum()
}

#[derive(Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    Joker,
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}
use Card::*;

impl Card {
    fn parse(ch: char, js_as_jokers: bool) -> aoc::Result<Card> {
        Ok(match ch {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' if js_as_jokers => Joker,
            'J' => Jack,
            '2'..='9' => Number(ch as u32 - '0' as u32),
            'T' => Number(10),
            _ => bail!("invalid character '{ch}'"),
        })
    }
}

impl Hand {
    fn new(cards: [Card; 5]) -> Hand {
        Hand {
            hand_type: Hand::hand_type(cards),
            cards,
        }
    }

    fn hand_type(cards: [Card; 5]) -> HandType {
        let cards = if cards.contains(&Joker) {
            Hand::replace_jokers_for_strongest_hand_type(cards)
        } else {
            cards
        };

        let mut counts: Vec<_> = cards.into_iter().counts().into_values().collect();
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

    fn replace_jokers_for_strongest_hand_type(cards: [Card; 5]) -> [Card; 5] {
        let counts = cards.into_iter().filter(|c| *c != Joker).counts();
        let highest_count_card = counts
            .into_iter()
            .max_by_key(|(_card, count)| *count)
            .map(|(card, _count)| card)
            // If cards are all jokers, replace them with a valid card, like aces.
            .unwrap_or(Ace);
        cards.map(|c| if c == Joker { highest_count_card } else { c })
    }
}

fn parse_line(line: &str, js_as_jokers: bool) -> aoc::Result<(Hand, u64)> {
    let (hand, bid) = line.split_once(' ').context("invalid input")?;
    let cards: Vec<_> = hand
        .chars()
        .map(|ch| Card::parse(ch, js_as_jokers))
        .try_collect()?;
    let hand = Hand::new(cards[..].try_into()?);
    Ok((hand, bid.parse()?))
}
