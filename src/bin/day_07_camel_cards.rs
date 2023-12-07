use itertools::Itertools;
use std::cmp::Ordering;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let mut hands: Vec<_> = input.lines().map(parse_line).try_collect()?;

    hands.sort_by_key(|(hand, _bid)| *hand);
    let ans_1 = total_winnings(&hands);

    hands.sort_by(|a, b| a.0.cmp_with_jokers(&b.0));
    let ans_2 = total_winnings(&hands);

    println!("{ans_1} {ans_2}");
    Ok(())
}

fn total_winnings(hands: &[(Hand, u64)]) -> u64 {
    hands
        .iter()
        .enumerate()
        .map(|(i, &(_hand, bid))| bid * (i + 1) as u64)
        .sum()
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

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then_with(|| self.cards.map(card_value).cmp(&other.cards.map(card_value)))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
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

    fn hand_type_with_jokers(&self) -> HandType {
        let counts = self.cards.into_iter().filter(|c| *c != 'J').counts();
        let highest_count_card = counts
            .iter()
            .max_by_key(|(_card, count)| **count)
            .map(|(card, _count)| *card)
            // If hand consists of just jokers, replace them with a valid label, like aces.
            .unwrap_or('A');
        let hand_with_jokers_replaced = Hand {
            cards: self
                .cards
                .map(|c| if c == 'J' { highest_count_card } else { c }),
        };
        hand_with_jokers_replaced.hand_type()
    }

    fn cmp_with_jokers(&self, other: &Hand) -> Ordering {
        self.hand_type_with_jokers()
            .cmp(&other.hand_type_with_jokers())
            .then_with(|| {
                self.cards
                    .map(card_value_with_jokers)
                    .cmp(&other.cards.map(card_value_with_jokers))
            })
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

fn card_value_with_jokers(card_label: char) -> usize {
    [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
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
