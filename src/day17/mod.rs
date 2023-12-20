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

#[derive(Debug, Clone)]
struct Location {
    cost: u32,
    loc: IVec2,
    from: Vec<Direction>,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Location {}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: &str) -> Answer {
    let (_, graph) = parse(input)?;

    let mut been: Vec<Vec<[u32; 4]>> = graph.iter().map(|x| vec![[u32::MAX; 4]; x.len()]).collect();
    let mut heap: BinaryHeap<Location> = BinaryHeap::new();

    let end = IVec2::new(graph[0].len() as i32 - 1, graph.len() as i32 - 1);

    let in_bounds = |loc: IVec2| loc.min_element() >= 0 && loc.x <= end.x && loc.y <= end.y;
    let idx = |d: Direction| match d {
        Direction::Up => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 3,
    };

    been[0][0] = [0; 4];
    heap.push(Location {
        cost: 0,
        loc: IVec2::ZERO,
        from: Vec::new(),
    });

    while let Some(place) = heap.pop() {
        // dbg!(&place);
        if place.loc == end {
            dbg!(place.from);
            dbg!(place.cost);
            // continue;
            return Ok(place.cost.to_string());
        }

        let b = been[place.loc.y as usize][place.loc.x as usize];
        if !place.from.is_empty() && place.cost > b[idx(place.from[place.from.len() - 1])] {
            continue;
        }

        for next in [
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ] {
            if place.from.last()
                == Some(&match next {
                    Direction::Up => Direction::Down,
                    Direction::Down => Direction::Up,
                    Direction::Left => Direction::Right,
                    Direction::Right => Direction::Left,
                })
            {
                continue;
            }
            let next_loc = place.loc + next.movement_vector();
            if !in_bounds(next_loc) {
                continue;
            }
            let mut arr = place.from.clone();
            let i = if arr.len() >= 3 {
                arr.iter()
                    .rev()
                    .take(3)
                    .all_equal()
                    .then_some(*arr.last().unwrap())
            } else {
                None
            };

            // four in a row
            if i == Some(next) {
                continue;
            }
            arr.push(next);
            let cost = place.cost + graph[next_loc.y as usize][next_loc.x as usize] as u32;
            let current_cost = *been[next_loc.y as usize][next_loc.x as usize]
                .iter()
                .max()
                .unwrap();

            if cost < current_cost {
                heap.push(Location {
                    cost,
                    loc: next_loc,
                    from: arr,
                });
                for (ind, x) in been[next_loc.y as usize][next_loc.x as usize]
                    .iter_mut()
                    .enumerate()
                {
                    if cost < *x && idx(next) != ind {
                        *x = cost;
                    }
                }
            }
        }
    }

    panic!("couldn't find answer")
}

pub fn part2(_input: &str) -> Answer {
    todo!()
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    separated_list1(
        line_ending,
        many1(
            take_while_m_n(1, 1, |x: char| !x.is_whitespace())
                .map(|c: &str| c.parse::<u8>().unwrap()),
        ),
    )(input)
}
