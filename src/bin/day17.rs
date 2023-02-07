use advent_of_code_2019::intcode::{Computer, RunResult};
use aoc_helpers::prelude::*;

struct Day17;

impl Problem for Day17 {
    type Input = Computer;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut camera_output = String::new();
        let mut c = input.clone();
        loop {
            match c.run(None).expect("program should be correct") {
                RunResult::Finished => break,
                RunResult::WaitingForInput => panic!(),
                RunResult::Output(output) => camera_output.push(output as u8 as char),
            }
        }

        let map: Vec<Vec<bool>> = camera_output.lines().map(|line| line.chars().map(|c| c != '.').collect()).collect();
        let mut alignment = 0;
        for (row_idx, row) in map.iter().enumerate() {
            for (col_idx, scaffold) in row.iter().enumerate() {
                if *scaffold && row_idx > 0 && col_idx > 0 && row_idx < map.len() - 1 && col_idx < row.len() - 1 && row[col_idx - 1] && row[col_idx + 1] && map[row_idx - 1][col_idx] && map[row_idx + 1][col_idx] {
                    alignment += row_idx * col_idx;
                }
            }
        }
        alignment
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        Default::default()
    }
}

fn main() {
    solve::<Day17>(include_str!("../../inputs/day17.txt"));
}
