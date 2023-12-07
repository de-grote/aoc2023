use crate::prelude::*;
use glam::I64Vec2;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Ranges {
    destination: i64,
    source: i64,
    length: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RangesOrSeed {
    Range(Ranges),
    Seed(I64Vec2),
}

impl RangesOrSeed {
    fn begin(&self) -> i64 {
        match self {
            RangesOrSeed::Range(x) => x.source,
            RangesOrSeed::Seed(x) => x.x,
        }
    }
}

impl PartialOrd for RangesOrSeed {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RangesOrSeed {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.begin().cmp(&other.begin())
    }
}

impl Ranges {
    fn translate(&self, i: i64) -> Option<i64> {
        if (self.source..=self.end()).contains(&i) {
            Some(i + self.translation_amount())
        } else {
            None
        }
    }

    fn translation_amount(&self) -> i64 {
        self.destination - self.source
    }

    fn end(&self) -> i64 {
        self.source + self.length - 1
    }
}

pub fn part1(input: &str) -> Answer {
    let (_, (mut seeds, ranges)) = parse(input)?;

    for r in ranges {
        for seed in seeds.iter_mut() {
            for range in r.iter() {
                if let Some(x) = range.translate(*seed) {
                    *seed = x;
                    break;
                };
            }
        }
    }

    let result = seeds.into_iter().min().unwrap();
    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, (mut seeds, ranges)) = parse_part2(input)?;

    for mut range in ranges {
        range.sort_by(|f, s| f.source.cmp(&s.source));
        let mut new_seeds = Vec::new();

        let mut latest_range: Option<Ranges> = None;
        let mut latest_seed: Option<I64Vec2> = None;
        for ros in range
            .into_iter()
            .map(RangesOrSeed::Range)
            .chain(
                merge_ranges(seeds.into_iter())
                    .into_iter()
                    .map(RangesOrSeed::Seed),
            )
            .sorted()
        {
            match ros {
                RangesOrSeed::Range(r) => {
                    if let Some(s) = latest_seed {
                        if s.y >= r.source {
                            if s.x != r.source {
                                new_seeds.push(I64Vec2::new(s.x, r.source - 1));
                            }
                            if r.end() < s.y {
                                new_seeds
                                    .push(I64Vec2::new(r.source, r.end()) + r.translation_amount());
                                latest_seed = Some(I64Vec2::new(r.end() + 1, s.y));
                            } else {
                                new_seeds
                                    .push(I64Vec2::new(r.source, s.y) + r.translation_amount());
                                latest_seed = None;
                            }
                        } else {
                            new_seeds.push(s);
                            latest_seed = None;
                        }
                    }
                    latest_range = Some(r);
                }
                RangesOrSeed::Seed(s) => {
                    if let Some(x) = latest_seed {
                        new_seeds.push(x);
                    }
                    if let Some(r) = latest_range {
                        if r.end() >= s.x {
                            if r.end() >= s.y {
                                new_seeds.push(s + r.translation_amount());
                                latest_seed = None;
                            } else {
                                new_seeds.push(I64Vec2::new(s.x, r.end()) + r.translation_amount());
                                latest_seed = Some(I64Vec2::new(r.end() + 1, s.y));
                            }
                        } else {
                            latest_seed = Some(s);
                        }
                    } else {
                        latest_seed = Some(s);
                    }
                }
            }
        }
        if let Some(s) = latest_seed {
            new_seeds.push(s);
        }
        // dbg!(&new_seeds);
        seeds = new_seeds;
    }

    let result = seeds.into_iter().map(|x| x.x).min().unwrap();
    Ok(result.to_string())
}

fn merge_ranges(it: impl Iterator<Item = I64Vec2>) -> Vec<I64Vec2> {
    it.sorted_by(|x, y| x.x.cmp(&y.x))
        .fold(Vec::new(), |mut acc: Vec<I64Vec2>, current| {
            let last = acc.last_mut();
            let Some(last) = last else {
                acc.push(current);
                return acc;
            };
            if last.y >= current.x - 1 {
                last.y = current.y.max(last.y);
            } else {
                acc.push(current);
            }

            acc
        })
}

fn parse(input: &str) -> IResult<&str, (Vec<i64>, Vec<Vec<Ranges>>)> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(space1, complete::i64),
        multispace1,
    )(input)?;

    let (input, ranges) = separated_list1(multispace1, parse_map)(input)?;

    Ok((input, (seeds, ranges)))
}

fn parse_part2(input: &str) -> IResult<&str, (Vec<I64Vec2>, Vec<Vec<Ranges>>)> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(
            space1,
            separated_pair(complete::i64, space1, complete::i64)
                .map(|(x, y)| glam::I64Vec2::new(x, x + y - 1)),
        ),
        multispace1,
    )(input)?;

    let (input, ranges) = separated_list1(multispace1, parse_map)(input)?;

    Ok((input, (seeds, ranges)))
}

fn parse_map(input: &str) -> IResult<&str, Vec<Ranges>> {
    let (input, _) = tuple((
        take_till1(|c: char| c.is_whitespace()),
        space1,
        take_till1(|c: char| c.is_whitespace()),
        line_ending,
    ))(input)?;
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Ranges> {
    let (input, destination) = terminated(complete::i64, space1)(input)?;
    let (input, (source, length)) = separated_pair(complete::i64, space1, complete::i64)(input)?;
    Ok((
        input,
        Ranges {
            destination,
            source,
            length,
        },
    ))
}
