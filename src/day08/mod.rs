use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

pub fn part1(input: &str) -> Answer {
    let (_, (directions, map)) = parse(input)?;

    let mut current = "AAA";
    let mut i = 0;

    while current != "ZZZ" {
        let direction = directions[i % directions.len()];
        let options = map.get(current).unwrap();
        current = match direction {
            Direction::Left => options.0,
            Direction::Right => options.1,
        };
        i += 1;
    }

    Ok(i.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, (directions, map)) = parse(input)?;

    let mut current = map
        .keys()
        .copied()
        .filter(|x| x.ends_with('A'))
        .enumerate()
        .collect_vec();
    let mut i = 0;
    let mut first_been = vec![0; current.len()];
    let mut rotation_amount = vec![0; current.len()];

    while rotation_amount.contains(&0) {
        let direction = directions[i % directions.len()];
        current = current
            .iter()
            .map(|&(i, c)| (i, map.get(c).unwrap()))
            .map(|(i, options)| {
                (
                    i,
                    match direction {
                        Direction::Left => options.0,
                        Direction::Right => options.1,
                    },
                )
            })
            .collect_vec();
        for done in current
            .iter()
            .filter_map(|&(i, v)| v.ends_with('Z').then_some(i))
        {
            if first_been[done] == 0 {
                first_been[done] = i;
            } else if rotation_amount[done] == 0 {
                rotation_amount[done] = i - first_been[done];
            }
        }
        i += 1;
    }
    // dbg!(&first_been, &rotation_amount); // <-- uncomment this to see why the next line works
    let result = rotation_amount.into_iter().map(|x| x as u64).fold(1, lcm);
    Ok(result.to_string())
}

#[allow(clippy::type_complexity)] // dont care
fn parse(input: &str) -> IResult<&str, (Vec<Direction>, BTreeMap<&str, (&str, &str)>)> {
    let (input, directions) = many1(alt((
        tag("L").map(|_| Direction::Left),
        tag("R").map(|_| Direction::Right),
    )))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, map) = separated_list1(
        line_ending,
        separated_pair(
            take(3usize),
            tuple((space1, tag("="), space1)),
            delimited(
                tag("("),
                separated_pair(take(3usize), tag(", "), take(3usize)),
                tag(")"),
            ),
        ),
    )(input)?;
    Ok((input, (directions, map.into_iter().collect())))
}
