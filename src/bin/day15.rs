use std::collections::{HashMap, HashSet};

use advent_of_code_2019::intcode::{Computer, RunResult};
use aoc_helpers::{
    prelude::*,
    walk::{Generator, Walker},
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

struct Day15;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Oxygen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            _ => Direction::East,
        }
    }
}

impl Direction {
    fn movement_command(&self) -> isize {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }

    fn movement_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, 1),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }
}

#[derive(Clone, Debug)]
struct Droid {
    computer: Computer,
    position: (isize, isize),
    tile: Tile,
    path_len: usize,
}

impl From<&Computer> for Droid {
    fn from(computer: &Computer) -> Self {
        let mut computer = computer.clone();
        assert!(matches!(computer.run(None), Ok(RunResult::WaitingForInput)));

        Self {
            computer,
            position: Default::default(),
            tile: Default::default(),
            path_len: Default::default(),
        }
    }
}

impl Droid {
    fn new_after_movement(&self, direction: Direction) -> Self {
        let position = (
            self.position.0 + direction.movement_offset().0,
            self.position.1 + direction.movement_offset().1,
        );
        let mut computer = self.computer.clone();
        let tile = match computer
            .run(Some(direction.movement_command()))
            .expect("program should be correct")
        {
            RunResult::Finished => panic!("shouldn't finish"),
            RunResult::WaitingForInput => panic!("shouldn't expect more input"),
            RunResult::Output(output) => match output {
                0 => {
                    // hit a wall
                    Tile::Wall
                }
                1 => {
                    // moved
                    Tile::Empty
                }
                2 => {
                    // moved onto oxygen system
                    Tile::Oxygen
                }
                _ => panic!("should output 0-2"),
            },
        };

        Self {
            computer,
            position,
            tile,
            path_len: self.path_len + 1,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct OxygenFindingWalker {
    visited: HashSet<(isize, isize)>,
}

struct NextStepGenerator {
    droid: Droid,
    unknown_directions: Vec<Direction>,
}

impl Generator<Droid> for NextStepGenerator {
    fn generate<F: FnMut(Droid)>(&mut self, mut callback: F) {
        for direction in &self.unknown_directions {
            callback(self.droid.new_after_movement(*direction));
        }
    }
}

impl Walker<Droid> for OxygenFindingWalker {
    type NextGenerator = NextStepGenerator;

    type Result = usize;

    fn visit(&mut self, state: &Droid) -> walk::VisitDecision<Self::Result, Self::NextGenerator> {
        self.visited.insert(state.position);

        match state.tile {
            Tile::Empty => {
                let (x, y) = state.position;
                let droid = state.clone();
                let unknown_directions: Vec<Direction> = ALL_DIRECTIONS
                    .iter()
                    .filter(|d| {
                        let (dx, dy) = d.movement_offset();
                        let p = (x + dx, y + dy);
                        !self.visited.contains(&p)
                    })
                    .copied()
                    .collect();
                walk::VisitDecision::Next(NextStepGenerator {
                    droid,
                    unknown_directions,
                })
            }
            Tile::Wall => walk::VisitDecision::Continue,
            Tile::Oxygen => walk::VisitDecision::Break(state.path_len),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct MapBuildingWalker {
    map: HashMap<(isize, isize), Tile>,
    oxygen_system: Option<(isize, isize)>,
}

impl Walker<Droid> for MapBuildingWalker {
    type NextGenerator = NextStepGenerator;

    type Result = ();

    fn visit(&mut self, state: &Droid) -> walk::VisitDecision<Self::Result, Self::NextGenerator> {
        self.map.insert(state.position, state.tile);
        if state.tile == Tile::Oxygen {
            self.oxygen_system = Some(state.position);
        }

        match state.tile {
            Tile::Empty | Tile::Oxygen => {
                let (x, y) = state.position;
                let unknown_directions: Vec<Direction> = ALL_DIRECTIONS
                    .iter()
                    .filter(|d| {
                        let (dx, dy) = d.movement_offset();
                        let p = (x + dx, y + dy);
                        !self.map.contains_key(&p)
                    })
                    .copied()
                    .collect();
                if unknown_directions.is_empty() {
                    walk::VisitDecision::Continue
                } else {
                    let droid = state.clone();
                    walk::VisitDecision::Next(NextStepGenerator {
                        droid,
                        unknown_directions,
                    })
                }
            }
            Tile::Wall => walk::VisitDecision::Continue,
        }
    }
}

struct OxygenSpreadingWalker {
    map: HashMap<(isize, isize), Tile>,
    distance: HashMap<(isize, isize), usize>,
}

impl From<MapBuildingWalker> for OxygenSpreadingWalker {
    fn from(walker: MapBuildingWalker) -> Self {
        Self {
            map: walker.map,
            distance: Default::default(),
        }
    }
}

struct PointsGenerator(Vec<(usize, (isize, isize))>);

impl Generator<(usize, (isize, isize))> for PointsGenerator {
    fn generate<F: FnMut((usize, (isize, isize)))>(&mut self, mut callback: F) {
        for x in &self.0 {
            callback(*x);
        }
    }
}

impl Walker<(usize, (isize, isize))> for OxygenSpreadingWalker {
    type NextGenerator = PointsGenerator;

    type Result = ();

    fn visit(
        &mut self,
        (distance, point): &(usize, (isize, isize)),
    ) -> walk::VisitDecision<Self::Result, Self::NextGenerator> {
        self.distance.entry(*point).or_insert(*distance);

        let (x, y) = point;
        let points: Vec<(usize, (isize, isize))> = ALL_DIRECTIONS
            .iter()
            .map(Direction::movement_offset)
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|p| {
                self.map.get(p).unwrap_or(&Tile::Wall) != &Tile::Wall
                    && !self.distance.contains_key(p)
            })
            .map(|p| (distance + 1, p))
            .collect();
        if points.is_empty() {
            walk::VisitDecision::Continue
        } else {
            walk::VisitDecision::Next(PointsGenerator(points))
        }
    }
}

impl Problem for Day15 {
    type Input = Computer;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let droid: Droid = input.into();
        let mut walker: OxygenFindingWalker = Default::default();
        walk::walk_broad(&mut walker, droid).expect("there should be oxygen system")
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let droid: Droid = input.into();
        let mut walker: MapBuildingWalker = Default::default();
        walk::walk_broad(&mut walker, droid);
        let oxygen_system = walker.oxygen_system.expect("there should be oxygen system");
        let mut walker: OxygenSpreadingWalker = walker.into();
        walk::walk_broad(&mut walker, (0, oxygen_system));
        walker
            .distance
            .into_values()
            .max()
            .expect("there should be distances")
    }
}

fn main() {
    solve::<Day15>(include_str!("../../inputs/day15.txt"));
}
