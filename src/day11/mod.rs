use glam::I64Vec2;

use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let mut i = parse(input);

    i = i
        .into_iter()
        .sorted_by(|a, b| a.x.cmp(&b.x))
        .scan((0, 0), |(dif, last), v| {
            if v.x - *last > 1 {
                *dif += v.x - *last - 1;
            }
            *last = v.x;
            Some(I64Vec2::new(v.x + *dif, v.y))
        })
        .collect();

    i = i
        .into_iter()
        .sorted_by(|a, b| a.y.cmp(&b.y))
        .scan((0, 0), |(dif, last), v| {
            if v.y - *last > 1 {
                *dif += v.y - *last - 1;
            }
            *last = v.y;
            Some(I64Vec2::new(v.x, v.y + *dif))
        })
        .collect();

    let result = i
        .iter()
        .combinations(2)
        .map(|v| {
            let d = (*v[0] - *v[1]).abs();
            d.x + d.y
        })
        .sum::<i64>();

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let mut i = parse(input);

    i = i
        .into_iter()
        .sorted_by(|a, b| a.x.cmp(&b.x))
        .scan((0, 0), |(dif, last), v| {
            if v.x - *last > 1 {
                *dif += (v.x - *last - 1) * (1_000_000 - 1);
            }
            *last = v.x;
            Some(I64Vec2::new(v.x + *dif, v.y))
        })
        .collect();

    i = i
        .into_iter()
        .sorted_by(|a, b| a.y.cmp(&b.y))
        .scan((0, 0), |(dif, last), v| {
            if v.y - *last > 1 {
                *dif += (v.y - *last - 1) * (1_000_000 - 1);
            }
            *last = v.y;
            Some(I64Vec2::new(v.x, v.y + *dif))
        })
        .collect();

    let result = i
        .iter()
        .combinations(2)
        .map(|v| {
            let d = (*v[0] - *v[1]).abs();
            d.x + d.y
        })
        .sum::<i64>();

    Ok(result.to_string())
}

fn parse(input: &str) -> HashSet<I64Vec2> {
    let mut out = HashSet::new();
    for (y, row) in input.lines().enumerate() {
        for (x, space) in row.chars().enumerate() {
            if space == '#' {
                out.insert(I64Vec2::new(x as i64, y as i64));
            }
        }
    }
    out
}
