use std::collections::HashMap;

use advent_of_code_2019::intcode::{Computer, RunResult};
use aoc_helpers::prelude::*;

struct Day13;

const WITH_DISPLAY: bool = false;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<isize> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        usize::try_from(value)
            .map_err(|err| anyhow::anyhow!("Value {:?} out of range: {}", value, err))
            .and_then(|index| {
                [
                    Self::Empty,
                    Self::Wall,
                    Self::Block,
                    Self::Paddle,
                    Self::Ball,
                ]
                .get(index)
                .copied()
                .ok_or_else(|| anyhow::anyhow!("Value {:?} out of range", value))
            })
    }
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Block => '.',
            Tile::Paddle => '=',
            Tile::Ball => '*',
        }
    }
}

fn display(map: &HashMap<(isize, isize), Tile>) {
    let min_x = map.iter().map(|((x, _), _)| *x).min().unwrap();
    let max_x = map.iter().map(|((x, _), _)| *x).max().unwrap();
    let min_y = map.iter().map(|((_, y), _)| *y).min().unwrap();
    let max_y = map.iter().map(|((_, y), _)| *y).max().unwrap();
    for y in min_y..=max_y {
        println!();
        for x in min_x..=max_x {
            print!(
                "{}",
                map.get(&(x, y)).copied().unwrap_or_default().to_char()
            );
        }
    }
    println!();
}

impl Problem for Day13 {
    type Input = VecFromCommaSeparated<isize>;
    type Part1 = usize;
    type Part2 = isize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut c: Computer = input.as_slice().into();
        let mut map: HashMap<(isize, isize), Tile> = Default::default();
        loop {
            match (c.run(None), c.run(None), c.run(None)) {
                (
                    Ok(RunResult::Output(x)),
                    Ok(RunResult::Output(y)),
                    Ok(RunResult::Output(tile_id)),
                ) => {
                    map.insert((x, y), tile_id.try_into().expect("invalid tile"));
                }
                (Ok(RunResult::Finished), _, _) => break,
                _ => panic!("invalid program"),
            }
        }
        map.into_iter()
            .filter(|(_, tile)| *tile == Tile::Block)
            .count()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut last_score = 0;
        let mut ball_x: isize = 0;
        let mut paddle_x: isize = 0;

        let mut c: Computer = input.as_slice().into();
        *c.get_mem_mut(0) = 2;
        let mut map: HashMap<(isize, isize), Tile> = Default::default();
        loop {
            match (
                c.run(Some((ball_x - paddle_x).signum())),
                c.run(Some((ball_x - paddle_x).signum())),
                c.run(Some((ball_x - paddle_x).signum())),
            ) {
                (
                    Ok(RunResult::Output(x)),
                    Ok(RunResult::Output(y)),
                    Ok(RunResult::Output(tile_id_or_score)),
                ) => {
                    if x == -1 && y == 0 {
                        if WITH_DISPLAY {
                            println!("Score: {}", tile_id_or_score);
                        }
                        last_score = tile_id_or_score;
                    } else {
                        let tile = tile_id_or_score.try_into().expect("invalid tile");
                        if tile == Tile::Paddle {
                            paddle_x = x;
                        } else if tile == Tile::Ball {
                            ball_x = x;
                        }
                        map.insert((x, y), tile);
                        if WITH_DISPLAY {
                            display(&map);
                            std::thread::sleep(std::time::Duration::from_millis(10));
                        }
                    }
                }
                (Ok(RunResult::Finished), _, _) => break,
                _ => panic!("invalid program"),
            }
        }
        last_score
    }
}

fn main() {
    solve::<Day13>(include_str!("../../inputs/day13.txt"));
}
