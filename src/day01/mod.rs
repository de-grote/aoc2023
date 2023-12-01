use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, x) = parse(input)?;
    let result: u32 = x
        .into_iter()
        .map(|y| y.into_iter().flat_map(|s| s.chars()).collect::<Vec<_>>())
        .map(|y| {
            (y[0].to_string() + &y[y.len() - 1].to_string())
                .parse::<u32>()
                .unwrap()
        })
        .sum();
    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let result: i32 = input
        .lines()
        .map(|line| {
            let matches = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1",
                "2", "3", "4", "5", "6", "7", "8", "9",
            ];
            let first = matches
                .into_iter()
                .filter_map(|x| line.find(x))
                .min()
                .unwrap();
            let last = matches
                .into_iter()
                .filter_map(|x| line.rfind(x))
                .max()
                .unwrap();
            index_to_int(line, first) * 10 + index_to_int(line, last)
        })
        .sum();
    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    separated_list1(line_ending, digits)(input)
}

fn digits(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        take_till(cond),
        separated_list0(take_till1(cond), digit1),
        take_till(cond),
    )(input)
}

fn cond(x: char) -> bool {
    x.is_numeric() || x.is_whitespace()
}

fn index_to_int(line: &str, idx: usize) -> i32 {
    match line.chars().nth(idx).unwrap() {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'o' => 1,
        'e' => 8,
        'n' => 9,
        't' => match line.chars().nth(idx + 1).unwrap() {
            'w' => 2,
            'h' => 3,
            i => panic!("23 {}", i),
        },
        'f' => match line.chars().nth(idx + 1).unwrap() {
            'o' => 4,
            'i' => 5,
            i => panic!("45 {}", i),
        },
        's' => match line.chars().nth(idx + 1).unwrap() {
            'i' => 6,
            'e' => 7,
            i => panic!("67 {}", i),
        },
        i => panic!("{}", i),
    }
}
