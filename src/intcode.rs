use std::str::FromStr;

use aoc_helpers::anyhow;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelativeBase,
    Halt,
}

impl TryFrom<isize> for Opcode {
    type Error = anyhow::Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            3 => Ok(Self::Input),
            4 => Ok(Self::Output),
            5 => Ok(Self::JumpIfTrue),
            6 => Ok(Self::JumpIfFalse),
            7 => Ok(Self::LessThan),
            8 => Ok(Self::Equals),
            9 => Ok(Self::AdjustRelativeBase),
            99 => Ok(Self::Halt),
            _ => Err(anyhow::anyhow!("Unknown opcode: {}", value)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<isize> for Mode {
    type Error = anyhow::Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            2 => Ok(Self::Relative),
            _ => Err(anyhow::anyhow!("Unknown mode: {}", value)),
        }
    }
}

impl Mode {
    fn get(&self, computer: &Computer, offset: usize) -> isize {
        let immediate = computer.get_mem(computer.idx + offset);
        match self {
            Mode::Position => computer.get_mem(immediate as usize),
            Mode::Immediate => immediate,
            Mode::Relative => computer.get_mem((computer.relative_base + immediate) as usize),
        }
    }

    fn get_mut<'a>(&self, computer: &'a mut Computer, offset: usize) -> Option<&'a mut isize> {
        let immediate = computer.get_mem(computer.idx + offset);
        match self {
            Mode::Position => Some(computer.get_mem_mut(immediate as usize)),
            Mode::Immediate => None,
            Mode::Relative => {
                Some(computer.get_mem_mut((computer.relative_base + immediate) as usize))
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
    arg1_mode: Mode,
    arg2_mode: Mode,
    arg3_mode: Mode,
}

impl TryFrom<isize> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        let opcode = (value % 100).try_into()?;
        let arg1_mode = (value / 100 % 10).try_into()?;
        let arg2_mode = (value / 1000 % 10).try_into()?;
        let arg3_mode = (value / 10000 % 10).try_into()?;
        Ok(Self {
            opcode,
            arg1_mode,
            arg2_mode,
            arg3_mode,
        })
    }
}

impl Instruction {
    fn execute(&self, computer: &mut Computer) {
        match self.opcode {
            Opcode::Input | Opcode::Output | Opcode::AdjustRelativeBase | Opcode::Halt => panic!(),
            Opcode::Add => computer.mut_2args_into_3rd(self, |a, b| a + b),
            Opcode::Mul => computer.mut_2args_into_3rd(self, |a, b| a * b),
            Opcode::JumpIfTrue => computer.jump_if_1st_into_2nd(self, |a| a != 0),
            Opcode::JumpIfFalse => computer.jump_if_1st_into_2nd(self, |a| a == 0),
            Opcode::LessThan => computer.mut_2args_into_3rd(self, |a, b| if a < b { 1 } else { 0 }),
            Opcode::Equals => computer.mut_2args_into_3rd(self, |a, b| if a == b { 1 } else { 0 }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Computer {
    mem: Vec<isize>,
    idx: usize,
    relative_base: isize,
}

impl From<&[isize]> for Computer {
    fn from(mem: &[isize]) -> Self {
        Self {
            mem: mem.to_vec(),
            idx: 0,
            relative_base: 0,
        }
    }
}

impl FromStr for Computer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mem: Vec<isize> = s
            .split(',')
            .map(|n| {
                n.parse::<isize>()
                    .map_err(|err| anyhow::anyhow!("Parsing {:?} to int failed: {}", n, err))
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            mem,
            idx: 0,
            relative_base: 0,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunResult {
    Finished,
    WaitingForInput,
    Output(isize),
}

impl Computer {
    fn mut_2args_into_3rd<F: Fn(isize, isize) -> isize>(&mut self, instr: &Instruction, fun: F) {
        let a = instr.arg1_mode.get(self, 1);
        let b = instr.arg2_mode.get(self, 2);
        let result = fun(a, b);
        *instr.arg3_mode.get_mut(self, 3).unwrap() = result;
        self.idx += 4;
    }

    fn jump_if_1st_into_2nd<F: Fn(isize) -> bool>(&mut self, instr: &Instruction, fun: F) {
        let a = instr.arg1_mode.get(self, 1);
        let b = instr.arg2_mode.get(self, 2);
        self.idx = if fun(a) { b as usize } else { self.idx + 3 }
    }

    fn get_mem(&self, idx: usize) -> isize {
        self.mem.get(idx).copied().unwrap_or_default()
    }

    fn get_mem_mut(&mut self, idx: usize) -> &mut isize {
        if self.mem.len() <= idx {
            let additional = idx - self.mem.len() + 1;
            self.mem.reserve(additional);
            self.mem.extend(std::iter::repeat(0).take(additional));
        }
        self.mem.get_mut(idx).expect("should be long enough")
    }

    pub fn run(&mut self, mut input: Option<isize>) -> Result<RunResult, anyhow::Error> {
        while self.idx < self.mem.len() {
            let instr: Instruction = self.mem[self.idx].try_into()?;

            match instr.opcode {
                Opcode::Input => {
                    if let Some(input) = input.take() {
                        *instr.arg1_mode.get_mut(self, 1).unwrap() = input;
                        self.idx += 2;
                    } else {
                        return Ok(RunResult::WaitingForInput);
                    }
                }
                Opcode::Output => {
                    let output = instr.arg1_mode.get(self, 1);
                    self.idx += 2;
                    return Ok(RunResult::Output(output));
                }
                Opcode::AdjustRelativeBase => {
                    self.relative_base += instr.arg1_mode.get(self, 1);
                    self.idx += 2;
                }
                Opcode::Halt => {
                    return Ok(RunResult::Finished);
                }
                _ => {
                    instr.execute(self);
                }
            }
        }
        Ok(RunResult::Finished)
    }

    pub fn run_with_constant_input(
        &mut self,
        input: isize,
    ) -> Result<Option<isize>, anyhow::Error> {
        loop {
            match self.run(Some(input))? {
                RunResult::Finished => return Ok(None),
                RunResult::WaitingForInput => continue,
                RunResult::Output(output) => return Ok(Some(output)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_without_input(program: &str) -> Computer {
        let mut c: Computer = program.parse().unwrap();
        if let Ok(RunResult::Finished) = c.run(None) {
            c
        } else {
            panic!("run_without_input didn't finish correctly")
        }
    }

    #[test]
    fn test_simple_commands() {
        assert_eq!(run_without_input("1,0,0,0,99").mem, vec![2, 0, 0, 0, 99]);
        assert_eq!(run_without_input("2,3,0,3,99").mem, vec![2, 3, 0, 6, 99]);
        assert_eq!(
            run_without_input("2,4,4,5,99,0").mem,
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            run_without_input("1,1,1,4,99,5,6,0,99").mem,
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    fn run(program: &str, input: isize) -> isize {
        let mut c: Computer = program.parse().unwrap();
        c.run_with_constant_input(input).unwrap().unwrap()
    }

    #[test]
    fn test_comparisons() {
        assert_eq!(run("3,9,8,9,10,9,4,9,99,-1,8", 8), 1);
        assert_eq!(run("3,9,8,9,10,9,4,9,99,-1,8", 10), 0);

        assert_eq!(run("3,9,7,9,10,9,4,9,99,-1,8", 6), 1);
        assert_eq!(run("3,9,7,9,10,9,4,9,99,-1,8", 8), 0);
        assert_eq!(run("3,9,7,9,10,9,4,9,99,-1,8", 10), 0);

        assert_eq!(run("3,3,1108,-1,8,3,4,3,99", 8), 1);
        assert_eq!(run("3,3,1108,-1,8,3,4,3,99", 10), 0);

        assert_eq!(run("3,3,1107,-1,8,3,4,3,99", 6), 1);
        assert_eq!(run("3,3,1107,-1,8,3,4,3,99", 8), 0);
        assert_eq!(run("3,3,1107,-1,8,3,4,3,99", 10), 0);
    }

    #[test]
    fn test_day05_part2_sample() {
        const PROGRAM: &str = concat!(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,",
            "125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        );
        assert_eq!(run(PROGRAM, 7), 999);
        assert_eq!(run(PROGRAM, 8), 1000);
        assert_eq!(run(PROGRAM, 9), 1001);
    }

    #[test]
    fn test_jumps() {
        assert_eq!(run("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0), 0);
        assert_eq!(run("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 10), 1);

        assert_eq!(run("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0), 0);
        assert_eq!(run("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 10), 1);
    }

    #[test]
    fn test_quine() {
        const PROGRAM: &str = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut c: Computer = PROGRAM.parse().unwrap();
        let parsed_program = c.mem.clone();
        let mut output = Vec::new();
        loop {
            match c.run(None).unwrap() {
                RunResult::Finished => break,
                RunResult::WaitingForInput => panic!("shouldn't wait for input"),
                RunResult::Output(out) => output.push(out),
            }
        }
        assert_eq!(parsed_program, output);
    }

    #[test]
    fn test_bignum() {
        assert_eq!(
            run("1102,34915192,34915192,7,4,7,99,0", 0),
            34915192 * 34915192
        );
        assert_eq!(run("104,1125899906842624,99", 0), 1125899906842624);
    }
}
