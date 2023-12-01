use nom::{branch::alt, bytes::complete::tag, character::complete, multi::many0, IResult};
use std::{
    env,
    error::Error,
    fs::read_to_string,
    path::PathBuf,
    time::{Duration, Instant},
};

pub type Answer<'a> = Result<String, Box<dyn Error + 'a>>;

pub mod prelude;

#[cfg(test)]
mod test;

// TODO update daily
pub mod day01;

macro_rules! solution {
    ($day:ident, $part:ident) => {{
        let input = read_to_string(
            ["src", stringify!($day), "input.txt"]
                .iter()
                .collect::<PathBuf>(),
        )
        .unwrap();
        let start = Instant::now();
        let result = $day::$part(&input).unwrap();
        let end = Instant::now();
        (result, end - start)
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (result, duration) = if args.len() == 1 {
        // TODO update daily
        solution!(day01, part2)
    } else if args.len() == 3 {
        get_solution(&args)
    } else {
        panic!("invalid format: provide 2 or 0 arguments")
    };

    println!();
    println!("{}", result);
    println!();
    println!("solved in {:?}", duration)
}

fn get_solution(args: &[String]) -> (String, Duration) {
    let (_, (day, part)) =
        parse_args(args[1].as_str(), args[2].as_str()).expect("invalid format, couldn't parse");
    if !(1..=25).contains(&day) || !(1..=2).contains(&part) {
        panic!("invalid format: day or part number too high")
    }
    match (day, part) {
        // TODO update daily
        (1, 1) => solution!(day01, part1),
        (1, 2) => solution!(day01, part2),

        _ => (
            "This day is not solved by me yet".to_string(),
            Duration::ZERO,
        ),
    }
}

fn parse_args<'a>(day: &'a str, part: &'a str) -> IResult<&'a str, (u8, u8)> {
    let (day, _) = many0(tag("-"))(day)?;
    let (day, d) = alt((tag("day="), tag("d="), tag("part="), tag("p="), tag("")))(day)?;
    let (_, day) = complete::u8(day)?;
    let (part, _) = many0(tag("-"))(part)?;
    let (part, p) = alt((tag("day="), tag("d="), tag("part="), tag("p="), tag("")))(part)?;
    let (_, part) = complete::u8(part)?;
    Ok((
        "",
        if d.starts_with('p') && p.starts_with('d') {
            (part, day)
        } else {
            (day, part)
        },
    ))
}
