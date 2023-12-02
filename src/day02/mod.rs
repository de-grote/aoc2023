use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
struct Boxes {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn part1(input: &str) -> Answer {
    let (_, x) = parse(input)?;
    let mut result = 0;
    for (i, boxes) in x.into_iter().enumerate() {
        let game = i + 1;
        if boxes
            .into_iter()
            .all(|b| b.red <= 12 && b.green <= 13 && b.blue <= 14)
        {
            result += game;
        }
    }
    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, x) = parse(input)?;
    let mut result = 0;
    for boxes in x.into_iter() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for b in boxes {
            red = (b.red as u32).max(red);
            green = (b.green as u32).max(green);
            blue = (b.blue as u32).max(blue);
        }
        result += red * green * blue;
    }
    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Boxes>>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Boxes>> {
    let (input, _) = delimited(tag("Game "), complete::u8, tag(":"))(input)?;
    separated_list1(tag(";"), parse_box)(input)
}

fn parse_box(input: &str) -> IResult<&str, Boxes> {
    let (input, _) = tag(" ")(input)?;
    let (input, v) = separated_list1(
        tag(", "),
        separated_pair(
            complete::u8,
            tag(" "),
            alt((tag("red"), tag("green"), tag("blue"))),
        ),
    )(input)?;
    let mut b = Boxes::default();
    for (i, t) in v {
        match t {
            "red" => b.red += i,
            "green" => b.green += i,
            "blue" => b.blue += i,
            _ => panic!("invalid colour"),
        };
    }
    Ok((input, b))
}
