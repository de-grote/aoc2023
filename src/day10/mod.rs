use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Start,
    Empty,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Pipe {
    fn neighbours(self, location: IVec2, board: &Vec<Vec<Pipe>>) -> Vec<IVec2> {
        let mut r = vec![];
        // north
        let l = IVec2::new(location.x, location.y - 1);
        if self.connects_north() && board.at(l).is_some_and(|p| p.connects_south()) {
            r.push(l);
        }
        // south
        let l = IVec2::new(location.x, location.y + 1);
        if self.connects_south() && board.at(l).is_some_and(|p| p.connects_north()) {
            r.push(l);
        }
        // east
        let l = IVec2::new(location.x + 1, location.y);
        if self.connects_east() && board.at(l).is_some_and(|p| p.connects_west()) {
            r.push(l);
        }

        // west
        let l = IVec2::new(location.x - 1, location.y);
        if self.connects_west() && board.at(l).is_some_and(|p| p.connects_east()) {
            r.push(l);
        }

        r
    }

    fn connects_south(self) -> bool {
        matches!(
            self,
            Pipe::NorthSouth | Pipe::SouthEast | Pipe::SouthWest | Pipe::Start
        )
    }

    fn connects_north(self) -> bool {
        matches!(
            self,
            Pipe::NorthSouth | Pipe::NorthEast | Pipe::NorthWest | Pipe::Start
        )
    }

    fn connects_east(self) -> bool {
        matches!(
            self,
            Pipe::EastWest | Pipe::NorthEast | Pipe::SouthEast | Pipe::Start
        )
    }

    fn connects_west(self) -> bool {
        matches!(
            self,
            Pipe::EastWest | Pipe::NorthWest | Pipe::SouthWest | Pipe::Start
        )
    }
}

trait At<T> {
    fn at(&self, location: IVec2) -> Option<T>;
}

impl<T> At<T> for Vec<Vec<T>>
where
    T: Copy,
{
    fn at(&self, location: IVec2) -> Option<T> {
        if location.min_element() < 0 {
            return None;
        }
        self.get(location.y as usize)
            .and_then(|v| v.get(location.x as usize).copied())
    }
}

pub fn part1(input: &str) -> Answer {
    let i = parse(input);
    let mut been: HashSet<IVec2> = HashSet::new();
    let mut current: HashSet<IVec2> = HashSet::new();
    let start = i
        .iter()
        .enumerate()
        .find_map(|(y, x)| {
            x.iter()
                .copied()
                .find_position(|&v| v == Pipe::Start)
                .map(|(i, _)| IVec2::new(i as i32, y as i32))
        })
        .unwrap();

    current.insert(start);

    let mut result = 1;

    'outer: loop {
        let mut new = HashSet::new();

        for c in current
            .iter()
            .flat_map(|&x| i.at(x).map(|p| p.neighbours(x, &i)))
            .flatten()
        {
            if !been.contains(&c) && !new.insert(c) {
                break 'outer;
            }
        }
        if new.is_empty() {
            break;
        }
        been.extend(current);
        current = new;
        result += 1;
    }

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let mut i = parse(input);
    let mut been: HashSet<IVec2> = HashSet::new();
    let mut current: HashSet<IVec2> = HashSet::new();
    let start = i
        .iter()
        .enumerate()
        .find_map(|(y, x)| {
            x.iter()
                .copied()
                .find_position(|&v| v == Pipe::Start)
                .map(|(i, _)| IVec2::new(i as i32, y as i32))
        })
        .unwrap();

    current.insert(start);

    'outer: loop {
        let mut new = HashSet::new();

        for c in current
            .iter()
            .flat_map(|&x| i.at(x).map(|p| p.neighbours(x, &i)))
            .flatten()
        {
            if !been.contains(&c) && !new.insert(c) {
                been.extend(new);
                break 'outer;
            }
        }
        if new.is_empty() {
            break;
        }
        been.extend(current);
        current = new;
    }
    been.extend(current);

    // replace start
    let n = Pipe::Start.neighbours(start, &i);
    // north south east west
    i[start.y as usize][start.x as usize] = match (
        n.contains(&IVec2::new(start.x, start.y - 1)),
        n.contains(&IVec2::new(start.x, start.y + 1)),
        n.contains(&IVec2::new(start.x + 1, start.y)),
        n.contains(&IVec2::new(start.x - 1, start.y)),
    ) {
        (true, true, false, false) => Pipe::NorthSouth,
        (true, false, true, false) => Pipe::NorthEast,
        (true, false, false, true) => Pipe::NorthWest,
        (false, true, true, false) => Pipe::SouthEast,
        (false, true, false, true) => Pipe::SouthWest,
        (false, false, true, true) => Pipe::EastWest,
        _ => panic!("you should panic reading this line anyways."),
    };

    let mut result = 0;
    for (y, row) in i.iter().enumerate() {
        let mut mask = false;
        let mut start_of_wall_segment = Pipe::Empty;
        for (x, pipe) in row.iter().copied().enumerate() {
            let coord = IVec2::new(x as i32, y as i32);
            if been.contains(&coord) {
                // part of the wall
                if pipe == Pipe::NorthSouth {
                    mask = !mask;
                } else if pipe == Pipe::EastWest {
                    continue;
                } else if !pipe.connects_west() {
                    // new part of wall
                    // dbg!(pipe, (x, y), mask);
                    start_of_wall_segment = pipe;
                } else if !pipe.connects_east() {
                    // end of wall segment
                    // dbg!(pipe, (x, y), mask);
                    if start_of_wall_segment.connects_north() != pipe.connects_north() {
                        mask = !mask;
                    }
                }
            } else if mask {
                result += 1;
            }
        }
    }

    Ok(result.to_string())
}

fn parse(input: &str) -> Vec<Vec<Pipe>> {
    input
        .lines()
        .map(|v| {
            v.chars()
                .map(|c| match c {
                    '|' => Pipe::NorthSouth,
                    '-' => Pipe::EastWest,
                    '.' => Pipe::Empty,
                    'S' => Pipe::Start,
                    'L' => Pipe::NorthEast,
                    'J' => Pipe::NorthWest,
                    '7' => Pipe::SouthWest,
                    'F' => Pipe::SouthEast,
                    x => panic!("{}", x),
                })
                .collect::<Vec<Pipe>>()
        })
        .collect()
}
