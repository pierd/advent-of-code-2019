use std::collections::HashMap;

use advent_of_code_2019::intcode::{Computer, RunResult};
use aoc_helpers::permutations::permutations;
use aoc_helpers::prelude::*;

struct Day07;

impl Problem for Day07 {
    type Input = VecFromCommaSeparated<isize>;
    type Part1 = isize;
    type Part2 = isize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut cache: HashMap<(isize, isize), isize> = Default::default();
        let mut run = |phase: isize, inp: isize| -> isize {
            *cache.entry((phase, inp)).or_insert_with(|| {
                let mut c: Computer = input.as_slice().into();
                match c.run(Some(phase)).expect("should parse") {
                    RunResult::Finished => panic!(),
                    RunResult::WaitingForInput | RunResult::Output(_) => {}
                };
                if let RunResult::Output(output) = c.run(Some(inp)).expect("should parse") {
                    return output;
                }
                panic!()
            })
        };
        let mut best = isize::MIN;
        permutations(vec![0, 1, 2, 3, 4], |phases| {
            let output = phases
                .iter()
                .fold(0, |prev_output, phase| run(*phase, prev_output));
            if best < output {
                best = output;
            }
        });
        best
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let init_amp = |phase: &isize| -> Computer {
            let mut c: Computer = input.as_slice().into();
            match c.run(Some(*phase)).expect("should parse") {
                RunResult::Finished | RunResult::Output(_) => panic!(),
                RunResult::WaitingForInput => c,
            }
        };

        let mut best = isize::MIN;
        permutations(vec![5, 6, 7, 8, 9], |phases| {
            let mut amps: Vec<Computer> = phases.iter().map(init_amp).collect();
            let mut output = 0;
            let mut done = false;
            while !done {
                for amp in &mut amps {
                    match amp.run(Some(output)).expect("should parse") {
                        RunResult::Finished => done = true,
                        RunResult::WaitingForInput => panic!(),
                        RunResult::Output(new_output) => output = new_output,
                    }
                }
            }
            if best < output {
                best = output;
            }
        });
        best
    }
}

fn main() {
    solve::<Day07>(include_str!("../../inputs/day07.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    #[test]
    fn test_sample() {
        assert_eq!(
            solve_part1::<Day07>("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            43210
        );
        assert_eq!(
            solve_part1::<Day07>(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            ),
            54321
        );
        assert_eq!(solve_part1::<Day07>("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 65210);

        assert_eq!(solve_part2::<Day07>("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"), 139629729);
        assert_eq!(solve_part2::<Day07>("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), 18216);
    }
}
