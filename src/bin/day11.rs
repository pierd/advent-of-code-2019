use std::collections::HashSet;

use advent_of_code_2019::intcode::{Computer, RunResult};
use aoc_helpers::prelude::*;

struct Day11;

fn turn_left((x, y): (isize, isize)) -> (isize, isize) {
    (-y, x)
}

fn turn_right((x, y): (isize, isize)) -> (isize, isize) {
    (y, -x)
}

impl Problem for Day11 {
    type Input = VecFromCommaSeparated<isize>;
    type Part1 = usize;
    type Part2 = String;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut white_panels: HashSet<(isize, isize)> = Default::default();
        let mut painted_panels: HashSet<(isize, isize)> = Default::default();
        let mut position = (0, 0);
        let mut velocity = (0, 1);
        let mut c: Computer = input.as_slice().into();
        loop {
            let is_white = white_panels.contains(&position);
            match (c.run(Some(if is_white { 1 } else { 0 })), c.run(None)) {
                (
                    Ok(RunResult::Output(should_be_white)),
                    Ok(RunResult::Output(should_turn_right)),
                ) => {
                    assert!(should_be_white == 1 || should_be_white == 0);
                    assert!(should_turn_right == 1 || should_turn_right == 0);
                    let should_be_white = should_be_white == 1;
                    if is_white != should_be_white {
                        painted_panels.insert(position);
                    }
                    if should_be_white {
                        white_panels.insert(position);
                    } else {
                        white_panels.remove(&position);
                    }
                    velocity = [turn_left, turn_right][should_turn_right as usize](velocity);
                    position = (position.0 + velocity.0, position.1 + velocity.1);
                }
                (Ok(RunResult::Finished), _) => break,
                _ => panic!("invalid program"),
            }
        }
        painted_panels.len()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut white_panels: HashSet<(isize, isize)> = Default::default();
        let mut position = (0, 0);
        let mut velocity = (0, 1);
        let mut c: Computer = input.as_slice().into();
        white_panels.insert(position);
        loop {
            let is_white = white_panels.contains(&position);
            match (c.run(Some(if is_white { 1 } else { 0 })), c.run(None)) {
                (
                    Ok(RunResult::Output(should_be_white)),
                    Ok(RunResult::Output(should_turn_right)),
                ) => {
                    assert!(should_be_white == 1 || should_be_white == 0);
                    assert!(should_turn_right == 1 || should_turn_right == 0);
                    let should_be_white = should_be_white == 1;
                    if should_be_white {
                        white_panels.insert(position);
                    } else {
                        white_panels.remove(&position);
                    }
                    velocity = [turn_left, turn_right][should_turn_right as usize](velocity);
                    position = (position.0 + velocity.0, position.1 + velocity.1);
                }
                (Ok(RunResult::Finished), _) => break,
                _ => panic!("invalid program"),
            }
        }

        let mut output = String::new();
        let min_x = white_panels.iter().map(|(x, _)| *x).min().unwrap();
        let max_x = white_panels.iter().map(|(x, _)| *x).max().unwrap();
        let min_y = white_panels.iter().map(|(_, y)| *y).min().unwrap();
        let max_y = white_panels.iter().map(|(_, y)| *y).max().unwrap();
        for y in (min_y..=max_y).rev() {
            output.push('\n');
            for x in min_x..=max_x {
                output.push(if white_panels.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                });
            }
        }
        output
    }
}

fn main() {
    solve::<Day11>(include_str!("../../inputs/day11.txt"));
}
