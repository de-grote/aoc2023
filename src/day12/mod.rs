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
        .map(|(tiles, numbers)| combinations(tiles, numbers))
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
            combinations(tiles, numbers)
        })
        .sum::<u64>();

    Ok(result.to_string())
}

fn combinations(tiles: Vec<Tile>, numbers: Vec<u32>) -> u64 {
    // replace all empty space with 1 space
    let mut groups = tiles
        .split_inclusive(|&v| v == Tile::Empty)
        .filter(|x| !x.iter().all(|&t| t == Tile::Empty))
        .flatten()
        .copied()
        .collect_vec();

    if groups.last().is_some_and(|&t| t == Tile::Empty) {
        groups.pop();
    }

    optimise_right(&mut groups, &numbers);

    optimise_left(&mut groups, &numbers);

    // dbg!(&groups);

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

    let unknowns_left = groups.iter().filter(|&&x| x == Tile::Unknown).count();

    // we want to reduce unknowns_left and sum as much as possible for the best performence
    // println!("{} | {}", unknowns_left, sum);

    for i in (0..2u64.pow(unknowns_left as u32)).filter(|&i| i.count_ones() == sum) {
        let a2 = (0..unknowns_left).map(|b| match ((i >> b) & 1) == 0 {
            true => Tile::Empty,
            false => Tile::Filled,
        });
        let r = split_groups
            .iter()
            .cloned()
            .interleave(a2.map(|x| vec![x]))
            .flatten();
        // dbg!(r.clone().collect_vec());
        if valid_combination(r, &numbers) {
            res += 1;
        }
    }
    res
}

fn optimise_right(groups: &mut [Tile], numbers: &[u32]) -> Option<std::convert::Infallible> {
    let mut b = true;
    let mut current_number = numbers.len() - 1;
    let mut l = groups.len() - 1;
    'outer: while b {
        b = false;
        let mut first = false;
        let current = numbers[current_number] as usize;
        for i in 0..current {
            let tile = groups.get_mut(l - i)?;
            if *tile == Tile::Filled {
                b = true;
                if i == 0 {
                    first = true;
                }
            } else if *tile == Tile::Empty {
                groups[l - i..=l].fill(Tile::Empty);
                l -= i + 1;
                b = true;
                continue 'outer;
            }
            if b {
                *tile = Tile::Filled;
            }
        }
        if first {
            if let Some(tile) = groups.get_mut(l.checked_sub(current)?) {
                *tile = Tile::Empty;
            }
            l = l.checked_sub(current + 1)?;
            current_number = current_number.checked_sub(1)?;
        } else if b {
            let s = l.checked_sub(current)?;
            let slice = &groups[..=s];

            let nearest_empty = slice
                .iter()
                .rev()
                .find_position(|&&t| t == Tile::Empty)
                .map(|x| x.0)
                .unwrap_or(slice.len());
            let slice = &mut groups[s.checked_sub(nearest_empty)? + 1..=l];
            let len = slice.len() as i32;
            let overlap = current as i32 * 2 - len;
            if overlap <= 0 {
                break;
            }
            let inv_overlap = ((len - overlap) / 2) as usize;
            slice[inv_overlap..len as usize - inv_overlap].fill(Tile::Filled);

            l = l.checked_sub(current + nearest_empty + 1)?;
            current_number = current_number.checked_sub(1)?;

            if inv_overlap as u32 > numbers[current_number] {
                break;
            }
        }
    }
    None
}

fn optimise_left(groups: &mut [Tile], numbers: &[u32]) -> Option<std::convert::Infallible> {
    let mut b = true;
    let mut current_number = 0;
    let mut l = 0;
    'outer: while b {
        b = false;
        let mut first = false;
        let current = numbers[current_number] as usize;
        for i in 0..current {
            let tile = groups.get_mut(l + i)?;
            if *tile == Tile::Filled {
                b = true;
                if i == 0 {
                    first = true;
                }
            } else if *tile == Tile::Empty {
                groups[l..=l + i].fill(Tile::Empty);
                l += i + 1;
                b = true;
                continue 'outer;
            }
            if b {
                *tile = Tile::Filled;
            }
        }
        if first {
            if let Some(tile) = groups.get_mut(l + current) {
                *tile = Tile::Empty;
            }
            l += current + 1;
            current_number += 1;
            if current_number == numbers.len() || l >= groups.len() {
                break;
            }
        } else if b {
            if l + current >= groups.len() {
                break;
            }
            let slice = &groups[l + current..];

            let nearest_empty = slice
                .iter()
                .find_position(|&&t| t == Tile::Empty)
                .map(|x| x.0)
                .unwrap_or(slice.len());
            let slice = &mut groups[l..l + current + nearest_empty];
            let len = slice.len() as i32;
            let overlap = current as i32 * 2 - len;
            if overlap <= 0 {
                break;
            }
            let inv_overlap = ((len - overlap) / 2) as usize;
            slice[inv_overlap..len as usize - inv_overlap].fill(Tile::Filled);

            l += current + nearest_empty + 1;
            current_number += 1;
            if l >= groups.len() || current_number == numbers.len() {
                break;
            }
            if inv_overlap as u32 > numbers[current_number] {
                break;
            }
        }
    }
    None
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
