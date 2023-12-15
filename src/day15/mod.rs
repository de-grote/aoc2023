use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, i) = parse(input)?;

    let result = i.into_iter().map(|c| hash(c) as u32).sum::<u32>();

    Ok(result.to_string())
}

fn hash(c: &[u8]) -> u8 {
    c.iter()
        .fold(0, |hash, &byte| hash.wrapping_add(byte).wrapping_mul(17))
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a [u8],
    operation: Operation,
}

#[derive(Debug)]
struct StoredLens<'a> {
    label: &'a [u8],
    focal_length: u8,
}

#[derive(Debug)]
enum Operation {
    Equals(u8),
    Dash,
}

pub fn part2(input: &str) -> Answer {
    let (_, lenses) = parse_part2(input)?;

    // this is a hashmap trust
    let mut hashmap = BTreeMap::<u8, Vec<StoredLens>>::new();

    for lens in lenses {
        let hash = hash(lens.label);
        match lens.operation {
            Operation::Equals(v) => {
                let boxes = hashmap.entry(hash).or_default();
                match boxes.iter_mut().find(|x| x.label == lens.label) {
                    Some(l) => l.focal_length = v,
                    None => boxes.push(StoredLens {
                        label: lens.label,
                        focal_length: v,
                    }),
                }
            }
            Operation::Dash => {
                if let Some(boxes) = hashmap.get_mut(&hash) {
                    if let Some((pos, _)) = boxes.iter().find_position(|x| x.label == lens.label) {
                        boxes.remove(pos);
                    }
                }
            }
        }
    }

    let mut result = 0;

    for (hash, lenses) in hashmap {
        for (pos, lens) in lenses.into_iter().enumerate() {
            result += (hash as u32 + 1) * (pos as u32 + 1) * lens.focal_length as u32;
        }
    }

    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<&[u8]>> {
    separated_list1(tag(","), is_not(",").map(|c: &str| c.as_bytes()))(input)
}

fn parse_part2(input: &str) -> IResult<&str, Vec<Lens>> {
    separated_list1(
        tag(","),
        pair(
            is_not("-=,").map(|c: &str| c.as_bytes()),
            alt((
                tag("-").map(|_| Operation::Dash),
                preceded(tag("="), complete::u8).map(Operation::Equals),
            )),
        )
        .map(|(label, operation)| Lens { label, operation }),
    )(input)
}
