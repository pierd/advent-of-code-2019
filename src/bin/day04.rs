use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day04;

#[derive(Clone, Copy, Debug)]
#[rematch(r"(\d+)-(\d+)")]
struct Spec {
    begin: usize,
    end: usize,
}

const fn digits(p: &usize) -> [usize; 6] {
    [
        *p / 100000,
        *p / 10000 % 10,
        *p / 1000 % 10,
        *p / 100 % 10,
        *p / 10 % 10,
        *p % 10,
    ]
}

fn is_valid(p: &usize) -> bool {
    let d = digits(p);
    d.windows(2).all(|w| w[0] <= w[1]) && d.windows(2).any(|w| w[0] == w[1])
}

fn is_valid_more(p: &usize) -> bool {
    let d = digits(p);
    d.windows(2).all(|w| w[0] <= w[1])
        && (d
            .windows(4)
            .any(|w| w[1] == w[2] && w[0] != w[1] && w[2] != w[3])
            || (d[0] == d[1] && d[1] != d[2])
            || (d[3] != d[4] && d[4] == d[5]))
}

impl Problem for Day04 {
    type Input = Spec;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        (input.begin..=input.end).filter(is_valid).count()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        (input.begin..=input.end).filter(is_valid_more).count()
    }
}

fn main() {
    solve::<Day04>("246515-739105");
}
