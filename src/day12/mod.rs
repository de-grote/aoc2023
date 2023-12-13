use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Filled,
    Unknown,
}

pub fn part1(input: &str) -> Answer {
    let (_, i) = parse(input)?;

    let result = i
        .into_iter()
        .map(|(tiles, numbers)| combinations(&tiles, numbers))
        .sum::<u64>();

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, i) = parse(input)?;

    let result = i
        .into_iter()
        .map(|(tiles, numbers)| {
            #[allow(unstable_name_collisions)]
            let tiles = repeat_n(tiles, 5)
                .intersperse(vec![Tile::Unknown])
                .flatten()
                .collect_vec();
            let numbers = numbers.repeat(5);
            combinations(&tiles, numbers)
        })
        .sum::<u64>();

    Ok(result.to_string())
}

fn combinations(tiles: &[Tile], numbers: Vec<u32>) -> u64 {
    let groups = tiles
        .split_inclusive(|&v| v == Tile::Empty)
        .filter(|x| !x.iter().all(|&t| t == Tile::Empty))
        .flatten()
        .copied()
        .collect_vec();

    let split_groups = groups
        .split(|&x| x == Tile::Unknown)
        .map(ToOwned::to_owned)
        .collect_vec();

    let filled = split_groups
        .iter()
        .flatten()
        .filter(|&&t| t == Tile::Filled)
        .count() as u32;
    let sum = numbers.iter().sum::<u32>() - filled;

    let mut res = 0;

    let a = groups.iter().filter(|&&x| x == Tile::Unknown).count() as u32;

    for i in 0..2u64.pow(a) {
        if i.count_ones() != sum {
            continue;
        }
        let a2 = (0..a).map(|b| match ((i >> b) & 1) == 0 {
            true => Tile::Empty,
            false => Tile::Filled,
        });
        let r = split_groups
            .clone()
            .into_iter()
            .interleave(a2.map(|x| vec![x]))
            .flatten();
        // dbg!(r.clone().collect_vec());
        if valid_combination(r, &numbers) {
            res += 1;
        }
    }
    res
}

fn valid_combination(mut it: impl Iterator<Item = Tile>, numbers: &[u32]) -> bool {
    for &n in numbers {
        loop {
            let x = it.next();
            if let Some(x) = x {
                if x == Tile::Filled {
                    break;
                }
            } else {
                return false;
            }
        }
        for _ in 0..(n - 1) {
            if !it.next().is_some_and(|x| x == Tile::Filled) {
                return false;
            }
        }
        if it.next().is_some_and(|x| x == Tile::Filled) {
            return false;
        }
    }
    it.all(|x| x == Tile::Empty)
}

#[allow(clippy::type_complexity)]
fn parse(input: &str) -> IResult<&str, Vec<(Vec<Tile>, Vec<u32>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            many1(alt((
                tag(".").map(|_| Tile::Empty),
                tag("#").map(|_| Tile::Filled),
                tag("?").map(|_| Tile::Unknown),
            ))),
            space1,
            separated_list1(tag(","), complete::u32),
        ),
    )(input)
}
