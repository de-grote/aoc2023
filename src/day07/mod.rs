use std::cmp::Ordering;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
    Ace,
}

impl Card {
    fn cmp_part2(self, other: Self) -> Ordering {
        if (self == Card::Jack) ^ (other == Card::Jack) {
            if self == Card::Jack {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            self.cmp(&other)
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Hand(Card, Card, Card, Card, Card, bool);

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut map: BTreeMap<Card, u8> = BTreeMap::new();
        for card in [self.0, self.1, self.2, self.3, self.4] {
            *map.entry(card).or_default() += 1;
        }
        if !self.5 || !map.contains_key(&Card::Jack) {
            // part 1 and part 2 if there are no jokers
            match map.len() {
                5 => HandType::HighCard,
                4 => HandType::OnePair,
                3 => {
                    if map.values().any(|&v| v == 3) {
                        HandType::ThreeOfAKind
                    } else {
                        HandType::TwoPair
                    }
                }
                2 => {
                    if map.values().any(|&v| v == 4) {
                        HandType::FourOfAKind
                    } else {
                        HandType::FullHouse
                    }
                }
                1 => HandType::FiveOfAKind,
                _ => panic!(),
            }
        } else {
            // part 2 with jokers
            let jokers = *map.get(&Card::Jack).unwrap();
            let other_cards = map
                .iter()
                .filter_map(|(&c, &v)| (c != Card::Jack).then_some(v))
                .collect_vec();
            match other_cards.len() {
                0 | 1 => HandType::FiveOfAKind,
                2 => {
                    if other_cards.iter().max().unwrap() + jokers == 4 {
                        HandType::FourOfAKind
                    } else {
                        HandType::FullHouse
                    }
                }
                3 => HandType::ThreeOfAKind,
                4 => HandType::OnePair,
                _ => panic!(),
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type().cmp(&other.hand_type()).then_with(|| {
            if self.5 {
                self.0.cmp_part2(other.0)
            } else {
                self.0.cmp(&other.0)
            }
            .then_with(|| {
                if self.5 {
                    self.1.cmp_part2(other.1)
                } else {
                    self.1.cmp(&other.1)
                }
                .then_with(|| {
                    if self.5 {
                        self.2.cmp_part2(other.2)
                    } else {
                        self.2.cmp(&other.2)
                    }
                    .then_with(|| {
                        if self.5 {
                            self.3.cmp_part2(other.3)
                        } else {
                            self.3.cmp(&other.3)
                        }
                        .then_with(|| {
                            if self.5 {
                                self.4.cmp_part2(other.4)
                            } else {
                                self.4.cmp(&other.4)
                            }
                        })
                    })
                })
            })
        })
    }
}

pub fn part1(input: &str) -> Answer {
    let (_, mut hands) = parse(input)?;

    hands.sort_by(|x, y| x.0.cmp(&y.0));

    let result = hands
        .into_iter()
        .zip(1u32..)
        .map(|((_, bid), score)| bid * score)
        .sum::<u32>();

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, mut hands) = parse(input)?;

    for (h, _) in hands.iter_mut() {
        h.5 = true;
    }

    hands.sort_by(|x, y| x.0.cmp(&y.0));

    let result = hands
        .into_iter()
        .zip(1u32..)
        .map(|((_, bid), score)| bid * score)
        .sum::<u32>();

    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<(Hand, u32)>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (Hand, u32)> {
    let (input, hand) = take(5usize)(input)?;
    let (input, bid) = preceded(space1, complete::u32)(input)?;
    let hand = hand
        .chars()
        .map(|x| match x {
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
            other => panic!("{other}"),
        })
        .collect_vec();
    let hand = Hand(hand[0], hand[1], hand[2], hand[3], hand[4], false);
    Ok((input, (hand, bid)))
}
