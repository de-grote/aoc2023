use crate::prelude::*;

#[derive(Debug)]
struct TimeAndDistance {
    time: i64,
    distance: i64,
}

pub fn part1(input: &str) -> Answer {
    let (_, time_and_distance) = parse(input)?;

    // -x^2 + time * x - distance = 0
    // x = (b+-sqrt(b^2 + c)) / 2

    let result = time_and_distance
        .into_iter()
        .map(|t| {
            let d = (t.time * t.time - 4 * t.distance) as f64;

            let mut x0 = ((t.time as f64 + d.sqrt()) / 2.0).floor() as i64;
            let mut x1 = ((t.time as f64 - d.sqrt()) / 2.0).ceil() as i64;

            x0 -= if x0 * (t.time - x0) == t.distance {
                1
            } else {
                0
            };
            x1 += if x1 * (t.time - x1) == t.distance {
                1
            } else {
                0
            };
            x0 - x1 + 1
        })
        .product::<i64>();

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, t) = parse_part2(input)?;

    let result = {
        let d = (t.time * t.time - 4 * t.distance) as f64;

        let mut x0 = ((t.time as f64 + d.sqrt()) / 2.0).floor() as i64;
        let mut x1 = ((t.time as f64 - d.sqrt()) / 2.0).ceil() as i64;

        x0 -= if x0 * (t.time - x0) == t.distance {
            1
        } else {
            0
        };
        x1 += if x1 * (t.time - x1) == t.distance {
            1
        } else {
            0
        };
        x0 - x1 + 1
    };

    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<TimeAndDistance>> {
    let (input, _) = preceded(tag("Time:"), space1)(input)?;
    let (input, time) = separated_list1(space1, complete::i64)(input)?;
    let (input, _) = tuple((line_ending, tag("Distance:"), space1))(input)?;
    let (input, distance) = separated_list1(space1, complete::i64)(input)?;
    Ok((
        input,
        izip!(time, distance)
            .map(|(time, distance)| TimeAndDistance { time, distance })
            .collect_vec(),
    ))
}

fn parse_part2(input: &str) -> IResult<&str, TimeAndDistance> {
    let (input, _) = preceded(tag("Time:"), space1)(input)?;
    let (input, time) = separated_list1(space1, digit1)
        .map(|x| x.into_iter().collect::<String>().parse::<i64>().unwrap())
        .parse(input)?;
    let (input, _) = tuple((line_ending, tag("Distance:"), space1))(input)?;
    let (input, distance) = separated_list1(space1, digit1)
        .map(|x| x.into_iter().collect::<String>().parse::<i64>().unwrap())
        .parse(input)?;
    Ok((input, TimeAndDistance { time, distance }))
}
