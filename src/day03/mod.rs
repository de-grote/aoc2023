use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
struct NumberWithLocation {
    number: u32,
    start: Location,
    end: Location,
}

#[derive(Debug, Clone, Copy, Default)]
struct Location {
    x: u32,
    y: u32,
}

pub fn part1(input: &str) -> Answer {
    let (_, (numbers, symbols)) = parse(input)?;

    let result = numbers
        .into_iter()
        .map(|n| {
            if (n.start.x..=n.end.x).any(|x| {
                symbols
                    .iter()
                    .any(|&(_, s)| x.abs_diff(s.x) <= 1 && n.start.y.abs_diff(s.y) <= 1)
            }) {
                n.number
            } else {
                0
            }
        })
        .sum::<u32>();

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, (numbers, symbols)) = parse(input)?;

    let result = symbols
        .into_iter()
        .filter_map(|x| if x.0 == "*" { Some(x.1) } else { None })
        .filter_map(|location| {
            let v = numbers
                .iter()
                .filter_map(|&n| {
                    if (n.start.x..=n.end.x)
                        .any(|x| x.abs_diff(location.x) <= 1 && n.start.y.abs_diff(location.y) <= 1)
                    {
                        Some(n.number)
                    } else {
                        None
                    }
                })
                .collect_vec();
            if v.len() == 2 {
                Some(v.into_iter().product::<u32>())
            } else {
                None
            }
        })
        .sum::<u32>();

    Ok(result.to_string())
}

#[allow(clippy::type_complexity)] // cope
fn parse(input: &str) -> IResult<&str, (Vec<NumberWithLocation>, Vec<(&str, Location)>)> {
    let mut symbols = Vec::new();
    let v = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            let mut it = iterator(line, digit_or_symbol);
            let mut to_add = 0;
            let out = it
                .enumerate()
                .filter_map(|(x, s)| {
                    if s == "." {
                        return None;
                    } else if !s.chars().all(|c| c.is_numeric()) {
                        symbols.push((
                            s,
                            Location {
                                x: x as u32 + to_add,
                                y: y as u32,
                            },
                        ));
                        return None;
                    }
                    let number = s.parse::<u32>().ok()?;
                    let start = to_add;
                    to_add += s.len() as u32 - 1;
                    Some(NumberWithLocation {
                        number,
                        start: Location {
                            x: x as u32 + start,
                            y: y as u32,
                        },
                        end: Location {
                            x: x as u32 + to_add,
                            y: y as u32,
                        },
                    })
                })
                .collect_vec();
            let _ = it.finish().ok()?;
            Some(out)
        })
        .flatten()
        .collect_vec();

    Ok((input, (v, symbols)))
}

fn digit_or_symbol(input: &str) -> IResult<&str, &str> {
    alt((digit1, take(1usize)))(input)
}
