use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Filled,
}

pub fn part1(input: &str) -> Answer {
    let (_, i) = parse(input)?;

    let result = i
        .into_iter()
        .map(|board| {
            let rotated_board = (0..board[0].len())
                .map(|idx| board.iter().map(|b| b[idx]).collect_vec())
                .collect_vec();
            let r = board
                .iter()
                .tuple_windows()
                .enumerate()
                .find_map(|(i, (a, b))| (a == b && is_palindrome(&board, i)).then_some(i));
            if let Some(r) = r {
                return (r as u32 + 1) * 100;
            }

            let c = rotated_board
                .iter()
                .tuple_windows()
                .enumerate()
                .find_map(|(i, (a, b))| (a == b && is_palindrome(&rotated_board, i)).then_some(i));

            c.unwrap() as u32 + 1
        })
        .sum::<u32>();

    Ok(result.to_string())
}

fn is_palindrome(board: &[Vec<Tile>], row: usize) -> bool {
    (0..=row)
        .rev()
        .zip(row + 1..board.len())
        .skip(1)
        .all(|(i, j)| board[i] == board[j])
}

fn is_almost_palindrome(board: &[Vec<Tile>], row: usize) -> bool {
    let mut almost = false;
    (0..=row).rev().zip(row + 1..board.len()).all(|(i, j)| {
        board[i] == board[j] || {
            if !almost {
                let x = almost_equal(&board[i], &board[j]);
                almost = true;
                x
            } else {
                false
            }
        }
    }) && almost
}

fn almost_equal(i: &[Tile], j: &[Tile]) -> bool {
    let mut miss = false;
    for (x, y) in i.iter().zip(j) {
        if x == y {
            continue;
        }
        if miss {
            return false;
        }
        miss = true;
    }
    miss
}

pub fn part2(input: &str) -> Answer {
    let (_, i) = parse(input)?;

    let result = i
        .into_iter()
        .map(|board| {
            let rotated_board = (0..board[0].len())
                .map(|idx| board.iter().map(|b| b[idx]).collect_vec())
                .collect_vec();
            let r = board
                .iter()
                .enumerate()
                .find_map(|(i, _)| (is_almost_palindrome(&board, i)).then_some(i));
            if let Some(r) = r {
                return (r as u32 + 1) * 100;
            }

            let c = rotated_board
                .iter()
                .enumerate()
                .find_map(|(i, _)| (is_almost_palindrome(&rotated_board, i)).then_some(i));

            c.unwrap() as u32 + 1
        })
        .sum::<u32>();

    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Vec<Tile>>>> {
    separated_list1(
        multispace1,
        separated_list1(
            line_ending,
            many1(alt((
                tag("#").map(|_| Tile::Filled),
                tag(".").map(|_| Tile::Empty),
            ))),
        ),
    )(input)
}
