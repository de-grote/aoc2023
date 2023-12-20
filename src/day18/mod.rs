use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn movement_vector(&self) -> IVec2 {
        match *self {
            Direction::Up => IVec2::new(0, -1),
            Direction::Down => IVec2::new(0, 1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
        }
    }
}

pub fn part1(input: &str) -> Answer {
    let (_, v) = parse(input)?;
    let mut current = IVec2::ZERO;
    let mut edges = HashSet::new();
    for (direction, amount) in v {
        let new_location = current + direction.movement_vector() * amount as i32;
        for (x, y) in
            chain!(current.x..=new_location.x, new_location.x..=current.x).cartesian_product(
                chain!(current.y..=new_location.y, new_location.y..=current.y),
            )
        {
            edges.insert(IVec2::new(x, y));
        }
        current = new_location;
    }

    let min_x = edges.iter().map(|x| x.x).min().unwrap();
    let min_y = edges.iter().map(|x| x.y).min().unwrap();
    let max_x = edges.iter().map(|x| x.x).max().unwrap();
    let max_y = edges.iter().map(|x| x.y).max().unwrap();

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let transform = -IVec2::new(min_x, min_y);

    dbg!(min_x, min_y, max_x, max_y, width, height, transform);

    let mut arr = vec![vec![true; width as usize]; height as usize];
    let mut to_search: HashSet<IVec2> = chain!(
        (0..width).map(|x| IVec2::new(x, 0)),
        (0..width).map(|x| IVec2::new(x, height - 1)),
        (0..height).map(|y| IVec2::new(0, y)),
        (0..height).map(|y| IVec2::new(width - 1, y)),
    )
    .collect();

    while let Some(&loc) = to_search.iter().next() {
        to_search.remove(&loc);
        if edges.contains(&(loc - transform)) {
            continue;
        }
        arr[loc.y as usize][loc.x as usize] = false;

        for dir in [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ] {
            let loc = loc + dir.movement_vector();
            if loc.min_element() >= 0
                && loc.x < width
                && loc.y < height
                && arr[loc.y as usize][loc.x as usize]
            {
                to_search.insert(loc);
            }
        }
    }

    let result = arr
        .into_iter()
        .map(|x| x.iter().filter(|&&b| b).count())
        .sum::<usize>();

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, v) = parse_part2(input)?;
    let mut current = IVec2::ZERO;
    let mut edges = HashSet::new();
    for (direction, amount) in v {
        let new_location = current + direction.movement_vector() * amount as i32;
        for (x, y) in
            chain!(current.x..=new_location.x, new_location.x..=current.x).cartesian_product(
                chain!(current.y..=new_location.y, new_location.y..=current.y),
            )
        {
            edges.insert(IVec2::new(x, y));
        }
        current = new_location;
    }

    let min_x = edges.iter().map(|x| x.x).min().unwrap();
    let min_y = edges.iter().map(|x| x.y).min().unwrap();
    let max_x = edges.iter().map(|x| x.x).max().unwrap();
    let max_y = edges.iter().map(|x| x.y).max().unwrap();

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let transform = -IVec2::new(min_x, min_y);

    let mut arr = vec![vec![true; width as usize]; height as usize];
    let mut to_search: HashSet<IVec2> = chain!(
        (0..width).map(|x| IVec2::new(x, 0)),
        (0..width).map(|x| IVec2::new(x, height - 1)),
        (0..height).map(|y| IVec2::new(0, y)),
        (0..height).map(|y| IVec2::new(width - 1, y)),
    )
    .collect();

    while let Some(&loc) = to_search.iter().next() {
        to_search.remove(&loc);
        if edges.contains(&(loc - transform)) {
            continue;
        }
        arr[loc.y as usize][loc.x as usize] = false;

        for dir in [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ] {
            let loc = loc + dir.movement_vector();
            if loc.min_element() >= 0
                && loc.x < width
                && loc.y < height
                && arr[loc.y as usize][loc.x as usize]
            {
                to_search.insert(loc);
            }
        }
    }

    let result = arr
        .into_iter()
        .map(|x| x.iter().filter(|&&b| b).count())
        .sum::<usize>();

    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<(Direction, u8)>> {
    separated_list1(
        line_ending,
        terminated(
            tuple((
                alt((
                    tag("L").map(|_| Direction::Left),
                    tag("R").map(|_| Direction::Right),
                    tag("U").map(|_| Direction::Up),
                    tag("D").map(|_| Direction::Down),
                )),
                delimited(space1, complete::u8, space1),
            )),
            delimited(tag("("), parse_color, tag(")")),
        ),
    )(input)
}

fn parse_part2(input: &str) -> IResult<&str, Vec<(Direction, u32)>> {
    separated_list1(
        line_ending,
        preceded(
            tuple((
                alt((
                    tag("L").map(|_| Direction::Left),
                    tag("R").map(|_| Direction::Right),
                    tag("U").map(|_| Direction::Up),
                    tag("D").map(|_| Direction::Down),
                )),
                delimited(space1, complete::u8, space1),
            )),
            delimited(tag("("), parse_color, tag(")")),
        ),
    )(input)
}

fn parse_color(input: &str) -> IResult<&str, (Direction, u32)> {
    let (input, _) = tag("#")(input)?;
    let (input, hex) = hex_digit1(input)?;
    let last = match hex.chars().last().unwrap() {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        x => panic!("{}", x),
    };
    let h = u32::from_str_radix(&hex[..5], 16).unwrap();

    Ok((input, (last, h)))
}
