use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Rock {
    Empty,
    Round,
    Cube,
}

pub fn part1(input: &str) -> Answer {
    let (_, i) = parse(input)?;

    let mut out = i.clone();
    for r in out.iter_mut() {
        r.fill(Rock::Empty);
    }

    for (y, row) in i.into_iter().enumerate() {
        for (x, col) in row.into_iter().enumerate() {
            match col {
                Rock::Empty => (),
                Rock::Cube => {
                    out[y][x] = Rock::Cube;
                }
                Rock::Round => {
                    let mut last = y;
                    for y2 in (0..y).rev() {
                        if out[y2][x] == Rock::Empty {
                            last = y2;
                        } else {
                            break;
                        }
                    }
                    out[last][x] = Rock::Round;
                }
            }
        }
    }

    let mut result = 0;

    for (r, score) in out.iter().zip((1..=out.len()).rev()) {
        result += r.iter().filter(|&&x| x == Rock::Round).count() * score;
    }

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, mut i) = parse(input)?;

    let mut set: HashMap<Vec<Vec<Rock>>, usize> = HashMap::new();

    for rotation in 0.. {
        let x_len = i[0].len();
        let y_len = i.len();

        if rotation % 4 == 0 {
            let cycle = rotation / 4;
            // dbg!(cycle, rotation);
            // print_board(&i);
            if let Some(&v) = set.get(&i) {
                if (1_000_000_000 - cycle) % (cycle - v) == 0
                    && (1_000_000_000 - v) % (cycle - v) == 0
                {
                    // print_board(&i);
                    break;
                }
            } else {
                set.insert(i.clone(), cycle);
            }
        }

        let mut out = i.clone();
        for r in out.iter_mut() {
            r.fill(Rock::Empty);
        }

        match rotation % 4 {
            0 => {
                for (y, row) in i.into_iter().enumerate() {
                    for (x, col) in row.into_iter().enumerate() {
                        match col {
                            Rock::Empty => (),
                            Rock::Cube => {
                                out[y][x] = Rock::Cube;
                            }
                            Rock::Round => {
                                let mut last = y;
                                for y2 in (0..y).rev() {
                                    if out[y2][x] == Rock::Empty {
                                        last = y2;
                                    } else {
                                        break;
                                    }
                                }
                                out[last][x] = Rock::Round;
                            }
                        }
                    }
                }
            }
            1 => {
                for (y, row) in i.into_iter().enumerate() {
                    for (x, col) in row.into_iter().enumerate() {
                        match col {
                            Rock::Empty => (),
                            Rock::Cube => {
                                out[y][x] = Rock::Cube;
                            }
                            Rock::Round => {
                                let mut last = x;
                                for x2 in (0..x).rev() {
                                    if out[y][x2] == Rock::Empty {
                                        last = x2;
                                    } else {
                                        break;
                                    }
                                }
                                out[y][last] = Rock::Round;
                            }
                        }
                    }
                }
            }
            2 => {
                for (y, row) in i.into_iter().enumerate().rev() {
                    for (x, col) in row.into_iter().enumerate() {
                        match col {
                            Rock::Empty => (),
                            Rock::Cube => {
                                out[y][x] = Rock::Cube;
                            }
                            Rock::Round => {
                                let mut last = y;
                                // it looks the same as all the others but this one is bad...
                                #[allow(clippy::needless_range_loop)]
                                for y2 in y + 1..y_len {
                                    if out[y2][x] == Rock::Empty {
                                        last = y2;
                                    } else {
                                        break;
                                    }
                                }
                                out[last][x] = Rock::Round;
                            }
                        }
                    }
                }
            }
            3 => {
                for (y, row) in i.into_iter().enumerate() {
                    for (x, col) in row.into_iter().enumerate().rev() {
                        match col {
                            Rock::Empty => (),
                            Rock::Cube => {
                                out[y][x] = Rock::Cube;
                            }
                            Rock::Round => {
                                let mut last = x;
                                for x2 in x + 1..x_len {
                                    if out[y][x2] == Rock::Empty {
                                        last = x2;
                                    } else {
                                        break;
                                    }
                                }
                                out[y][last] = Rock::Round;
                            }
                        }
                    }
                }
            }
            _ => unreachable!(),
        }

        i = out;
    }

    let mut result = 0;

    for (r, score) in i.iter().zip((1..=i.len()).rev()) {
        result += r.iter().filter(|&&x| x == Rock::Round).count() * score;
    }

    Ok(result.to_string())
}

#[allow(dead_code)]
fn print_board(board: &[Vec<Rock>]) {
    for r in board {
        for &v in r {
            print!(
                "{}",
                match v {
                    Rock::Cube => "#",
                    Rock::Empty => ".",
                    Rock::Round => "O",
                }
            )
        }
        println!();
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Rock>>> {
    separated_list1(
        line_ending,
        many1(alt((
            tag("#").map(|_| Rock::Cube),
            tag(".").map(|_| Rock::Empty),
            tag("O").map(|_| Rock::Round),
        ))),
    )(input)
}
