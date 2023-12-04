use crate::prelude::*;

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    have: Vec<u32>,
}

pub fn part1(input: &str) -> Answer {
    let (_, cards) = parse(input)?;

    let result = cards
        .into_iter()
        .map(|card| {
            let n = card
                .have
                .iter()
                .filter(|&number| card.winning.contains(number))
                .count() as u32;
            if n == 0 {
                0
            } else {
                2u32.pow(n - 1)
            }
        })
        .sum::<u32>();

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, cards) = parse(input)?;

    let mut card_collection: BTreeMap<u32, u32> = BTreeMap::new();

    for (idx, card) in cards.iter().enumerate() {
        let n = card
            .have
            .iter()
            .filter(|&number| card.winning.contains(number))
            .count();
        let dups = *card_collection.entry(idx as u32).or_insert(1);
        for extra in (idx + 1)..=(idx + n) {
            *card_collection.entry(extra as u32).or_insert(1) += dups;
        }
    }

    Ok(card_collection.values().sum::<u32>().to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _i) = delimited(space1, complete::u32, preceded(tag(":"), space1))(input)?;
    let (input, (winning, have)) = separated_pair(
        separated_list1(space1, complete::u32),
        delimited(space1, tag("|"), space1),
        separated_list1(space1, complete::u32),
    )(input)?;
    Ok((input, Card { winning, have }))
}
