use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, i) = parse(input)?;

    let result = i
        .into_iter()
        .map(|history| {
            let mut last = vec![];
            let mut current = history;
            while current.iter().any(|&x| x != 0) {
                last.push(*current.last().unwrap());
                current = current
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| (b - a))
                    .collect();
            }
            last.into_iter().sum::<i32>()
        })
        .sum::<i32>();

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, i) = parse(input)?;

    let result = i
        .into_iter()
        .map(|history| {
            let mut first = vec![];
            let mut current = history;
            while current.iter().any(|&x| x != 0) {
                first.push(*current.first().unwrap());
                current = current
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| (b - a))
                    .collect();
            }
            first.into_iter().rev().fold(0, |acc, v| {v - acc})
        })
        .sum::<i32>();

    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, separated_list1(space1, complete::i32))(input)
}
