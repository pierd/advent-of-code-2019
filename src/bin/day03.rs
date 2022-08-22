use std::collections::{HashMap, HashSet};

use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day03;

#[derive(Clone, Copy, Debug)]
#[rematch]
enum Direction {
    #[rematch(r"U")]
    Up,
    #[rematch(r"D")]
    Down,
    #[rematch(r"L")]
    Left,
    #[rematch(r"R")]
    Right,
}

impl Direction {
    fn coords(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[rematch(r"([UDLR])(\d+)")]
struct Component {
    direction: Direction,
    distance: usize,
}

fn walk<F: FnMut((isize, isize))>(path: &[Component], mut callback: F) {
    let (mut row, mut col) = (0, 0);
    for Component {
        direction,
        distance,
    } in path
    {
        let (drow, dcol) = direction.coords();
        for _ in 0..*distance {
            row += drow;
            col += dcol;
            callback((row, col));
        }
    }
}

impl Problem for Day03 {
    type Input = VecFromLines<VecFromCommaSeparated<Component>>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut visited: HashSet<(isize, isize)> = Default::default();
        walk(input[0].as_slice(), |p| {
            visited.insert(p);
        });
        let mut closes = usize::MAX;
        walk(input[1].as_slice(), |p| {
            if visited.contains(&p) {
                let dist = p.0.unsigned_abs() + p.1.unsigned_abs();
                if dist < closes {
                    closes = dist;
                }
            }
        });
        closes
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut visited: HashMap<(isize, isize), usize> = Default::default();
        let mut step = 0;
        walk(input[0].as_slice(), |p| {
            step += 1;
            visited.insert(p, step);
        });
        let mut closes = usize::MAX;
        let mut step = 0;
        walk(input[1].as_slice(), |p| {
            step += 1;
            if let Some(other_step) = visited.get(&p) {
                let dist = other_step + step;
                if dist < closes {
                    closes = dist;
                }
            }
        });
        closes
    }
}

fn main() {
    solve::<Day03>(include_str!("../../inputs/day03.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day03>("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
        assert_eq!(
            solve_part1::<Day03>(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
        assert_eq!(
            solve_part1::<Day03>(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );

        assert_eq!(solve_part2::<Day03>("R8,U5,L5,D3\nU7,R6,D4,L4"), 30);
        assert_eq!(
            solve_part2::<Day03>(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            ),
            610
        );
        assert_eq!(
            solve_part2::<Day03>(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
