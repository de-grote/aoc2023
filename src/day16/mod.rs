use glam::IVec2;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Mirror {
    /// /
    Slash,
    /// \
    BackSlash,
    /// -
    HorizontalSplitter,
    /// |
    VerticalSplitter,
}

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
    let map = parse(input);
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let in_bounds =
        |location: IVec2| location.min_element() >= 0 && location.x < width && location.y < height;

    let mut been = HashSet::new();
    let mut current = HashSet::new();
    current.insert((IVec2::new(0, 0), Direction::Right));

    while !current.is_empty() {
        let mut new = HashSet::new();
        for (location, direction) in current {
            been.insert((location, direction));
            if let Some(&mirror) = map.get(&location) {
                match mirror {
                    Mirror::Slash => {
                        let new_direction = match direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                        };
                        let new_location = location + new_direction.movement_vector();
                        if in_bounds(new_location) && !been.contains(&(new_location, new_direction))
                        {
                            new.insert((new_location, new_direction));
                        }
                    }
                    Mirror::BackSlash => {
                        let new_direction = match direction {
                            Direction::Up => Direction::Left,
                            Direction::Down => Direction::Right,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                        };
                        let new_location = location + new_direction.movement_vector();
                        if in_bounds(new_location) && !been.contains(&(new_location, new_direction))
                        {
                            new.insert((new_location, new_direction));
                        }
                    }
                    Mirror::HorizontalSplitter => {
                        if matches!(direction, Direction::Right | Direction::Left) {
                            let new_location = location + direction.movement_vector();
                            if in_bounds(new_location) && !been.contains(&(new_location, direction))
                            {
                                new.insert((new_location, direction));
                            }
                        } else {
                            let new_location_1 = location + Direction::Left.movement_vector();
                            let new_location_2 = location + Direction::Right.movement_vector();
                            if in_bounds(new_location_1)
                                && !been.contains(&(new_location_1, Direction::Left))
                            {
                                new.insert((new_location_1, Direction::Left));
                            }
                            if in_bounds(new_location_2)
                                && !been.contains(&(new_location_2, Direction::Right))
                            {
                                new.insert((new_location_2, Direction::Right));
                            }
                        }
                    }
                    Mirror::VerticalSplitter => {
                        if matches!(direction, Direction::Up | Direction::Down) {
                            let new_location = location + direction.movement_vector();
                            if in_bounds(new_location) && !been.contains(&(new_location, direction))
                            {
                                new.insert((new_location, direction));
                            }
                        } else {
                            let new_location_1 = location + Direction::Up.movement_vector();
                            let new_location_2 = location + Direction::Down.movement_vector();
                            if in_bounds(new_location_1)
                                && !been.contains(&(new_location_1, Direction::Up))
                            {
                                new.insert((new_location_1, Direction::Up));
                            }
                            if in_bounds(new_location_2)
                                && !been.contains(&(new_location_2, Direction::Down))
                            {
                                new.insert((new_location_2, Direction::Down));
                            }
                        }
                    }
                }
            } else {
                let new_location = location + direction.movement_vector();
                if in_bounds(new_location) && !been.contains(&(new_location, direction)) {
                    new.insert((new_location, direction));
                }
            }
        }
        current = new;
    }

    let result = been.into_iter().map(|x| x.0).unique().count();

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let map = parse(input);
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let in_bounds =
        |location: IVec2| location.min_element() >= 0 && location.x < width && location.y < height;

    let result = [
        Direction::Left,
        Direction::Down,
        Direction::Right,
        Direction::Up,
    ]
    .into_iter()
    .map(|d| {
        (0..(if matches!(d, Direction::Down | Direction::Up) {
            width
        } else {
            height
        }))
            .map(|start| {
                let mut been = HashSet::new();
                let mut current = HashSet::new();
                current.insert((
                    IVec2::new(
                        match d {
                            Direction::Up | Direction::Down => start,
                            Direction::Left => width,
                            Direction::Right => 0,
                        },
                        match d {
                            Direction::Left | Direction::Right => start,
                            Direction::Up => height,
                            Direction::Down => 0,
                        },
                    ),
                    d,
                ));

                while !current.is_empty() {
                    let mut new = HashSet::new();
                    for (location, direction) in current {
                        been.insert((location, direction));
                        if let Some(&mirror) = map.get(&location) {
                            match mirror {
                                Mirror::Slash => {
                                    let new_direction = match direction {
                                        Direction::Up => Direction::Right,
                                        Direction::Down => Direction::Left,
                                        Direction::Left => Direction::Down,
                                        Direction::Right => Direction::Up,
                                    };
                                    let new_location = location + new_direction.movement_vector();
                                    if in_bounds(new_location)
                                        && !been.contains(&(new_location, new_direction))
                                    {
                                        new.insert((new_location, new_direction));
                                    }
                                }
                                Mirror::BackSlash => {
                                    let new_direction = match direction {
                                        Direction::Up => Direction::Left,
                                        Direction::Down => Direction::Right,
                                        Direction::Left => Direction::Up,
                                        Direction::Right => Direction::Down,
                                    };
                                    let new_location = location + new_direction.movement_vector();
                                    if in_bounds(new_location)
                                        && !been.contains(&(new_location, new_direction))
                                    {
                                        new.insert((new_location, new_direction));
                                    }
                                }
                                Mirror::HorizontalSplitter => {
                                    if matches!(direction, Direction::Right | Direction::Left) {
                                        let new_location = location + direction.movement_vector();
                                        if in_bounds(new_location)
                                            && !been.contains(&(new_location, direction))
                                        {
                                            new.insert((new_location, direction));
                                        }
                                    } else {
                                        let new_location_1 =
                                            location + Direction::Left.movement_vector();
                                        let new_location_2 =
                                            location + Direction::Right.movement_vector();
                                        if in_bounds(new_location_1)
                                            && !been.contains(&(new_location_1, Direction::Left))
                                        {
                                            new.insert((new_location_1, Direction::Left));
                                        }
                                        if in_bounds(new_location_2)
                                            && !been.contains(&(new_location_2, Direction::Right))
                                        {
                                            new.insert((new_location_2, Direction::Right));
                                        }
                                    }
                                }
                                Mirror::VerticalSplitter => {
                                    if matches!(direction, Direction::Up | Direction::Down) {
                                        let new_location = location + direction.movement_vector();
                                        if in_bounds(new_location)
                                            && !been.contains(&(new_location, direction))
                                        {
                                            new.insert((new_location, direction));
                                        }
                                    } else {
                                        let new_location_1 =
                                            location + Direction::Up.movement_vector();
                                        let new_location_2 =
                                            location + Direction::Down.movement_vector();
                                        if in_bounds(new_location_1)
                                            && !been.contains(&(new_location_1, Direction::Up))
                                        {
                                            new.insert((new_location_1, Direction::Up));
                                        }
                                        if in_bounds(new_location_2)
                                            && !been.contains(&(new_location_2, Direction::Down))
                                        {
                                            new.insert((new_location_2, Direction::Down));
                                        }
                                    }
                                }
                            }
                        } else {
                            let new_location = location + direction.movement_vector();
                            if in_bounds(new_location) && !been.contains(&(new_location, direction))
                            {
                                new.insert((new_location, direction));
                            }
                        }
                    }
                    current = new;
                }

                been.into_iter().map(|x| x.0).unique().count()
            })
            .max()
            .unwrap()
    })
    .max()
    .unwrap();

    Ok(result.to_string())
}

fn parse(input: &str) -> HashMap<IVec2, Mirror> {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(
                IVec2::new(x as i32, y as i32),
                match c {
                    '.' => continue,
                    '/' => Mirror::Slash,
                    '\\' => Mirror::BackSlash,
                    '|' => Mirror::VerticalSplitter,
                    '-' => Mirror::HorizontalSplitter,
                    _ => unreachable!("invalid input"),
                },
            );
        }
    }
    map
}
