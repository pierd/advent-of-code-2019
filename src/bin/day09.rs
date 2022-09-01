use advent_of_code_2019::intcode::Computer;
use aoc_helpers::prelude::*;

struct Day09;

impl Problem for Day09 {
    type Input = VecFromCommaSeparated<isize>;
    type Part1 = isize;
    type Part2 = isize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut c: Computer = input.as_slice().into();
        while let Ok(Some(output)) = c.run_with_constant_input(1) {
            if output != 0 {
                return output;
            }
        }
        panic!()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut c: Computer = input.as_slice().into();
        c.run_with_constant_input(2)
            .expect("should be a valid program")
            .expect("should output")
    }
}

fn main() {
    solve::<Day09>(include_str!("../../inputs/day09.txt"));
}
