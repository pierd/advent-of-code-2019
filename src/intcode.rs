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
            _ => Err(anyhow::anyhow!("Unknown opcode: {}", value)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
}

impl TryFrom<isize> for Mode {
    type Error = anyhow::Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => Err(anyhow::anyhow!("Unknown mode: {}", value)),
        }
    }
}

impl Mode {
    fn get(&self, immediate_val: isize, mem: &[isize]) -> isize {
        match self {
            Mode::Position => mem[immediate_val as usize],
            Mode::Immediate => immediate_val,
        }
    }

    fn get_mut<'a>(&self, immediate_val: isize, mem: &'a mut [isize]) -> &'a mut isize {
        match self {
            Mode::Position => mem.get_mut(immediate_val as usize).unwrap(),
            Mode::Immediate => panic!(),
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
    fn execute(&self, mem: &mut [isize], idx: usize) -> usize {
        match self.opcode {
            Opcode::Add => {
                let a = self.arg1_mode.get(mem[idx + 1], mem);
                let b = self.arg2_mode.get(mem[idx + 2], mem);
                *self.arg3_mode.get_mut(mem[idx + 3], mem) = a + b;
                idx + 4
            }
            Opcode::Mul => {
                let a = self.arg1_mode.get(mem[idx + 1], mem);
                let b = self.arg2_mode.get(mem[idx + 2], mem);
                *self.arg3_mode.get_mut(mem[idx + 3], mem) = a * b;
                idx + 4
            }
            Opcode::Input | Opcode::Output => panic!(),
            Opcode::JumpIfTrue => {
                let a = self.arg1_mode.get(mem[idx + 1], mem);
                let b = self.arg2_mode.get(mem[idx + 2], mem);
                if a != 0 {
                    b as usize
                } else {
                    idx + 3
                }
            }
            Opcode::JumpIfFalse => {
                let a = self.arg1_mode.get(mem[idx + 1], mem);
                let b = self.arg2_mode.get(mem[idx + 2], mem);
                if a == 0 {
                    b as usize
                } else {
                    idx + 3
                }
            }
            Opcode::LessThan => {
                let a = self.arg1_mode.get(mem[idx + 1], mem);
                let b = self.arg2_mode.get(mem[idx + 2], mem);
                *self.arg3_mode.get_mut(mem[idx + 3], mem) = if a < b { 1 } else { 0 };
                idx + 4
            }
            Opcode::Equals => {
                let a = self.arg1_mode.get(mem[idx + 1], mem);
                let b = self.arg2_mode.get(mem[idx + 2], mem);
                *self.arg3_mode.get_mut(mem[idx + 3], mem) = if a == b { 1 } else { 0 };
                idx + 4
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Computer {
    mem: Vec<isize>,
    idx: usize,
}

impl From<&[isize]> for Computer {
    fn from(mem: &[isize]) -> Self {
        Self {
            mem: mem.to_vec(),
            idx: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RunResult {
    Finished,
    WaitingForInput,
    Output(isize),
}

impl Computer {
    pub fn run(&mut self, mut input: Option<isize>) -> Result<RunResult, anyhow::Error> {
        while self.idx < self.mem.len() {
            let instr: Instruction = self.mem[self.idx].try_into()?;

            match instr.opcode {
                Opcode::Input => {
                    if let Some(input) = input.take() {
                        *instr
                            .arg1_mode
                            .get_mut(self.mem[self.idx + 1], &mut self.mem) = input;
                        self.idx += 2;
                    } else {
                        return Ok(RunResult::WaitingForInput);
                    }
                }
                Opcode::Output => {
                    let output = instr.arg1_mode.get(self.mem[self.idx + 1], &self.mem);
                    self.idx += 2;
                    return Ok(RunResult::Output(output));
                }
                _ => {
                    self.idx = instr.execute(&mut self.mem, self.idx);
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

    fn run(program: &str, input: isize) -> isize {
        let parsed: Vec<isize> = program
            .split(',')
            .map(|n| n.parse::<isize>().unwrap())
            .collect();
        let mut c: Computer = parsed.as_slice().into();
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
    fn test_jumps() {
        assert_eq!(run("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0), 0);
        assert_eq!(run("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 10), 1);

        assert_eq!(run("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0), 0);
        assert_eq!(run("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 10), 1);
    }
}
