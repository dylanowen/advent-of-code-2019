use std::collections::VecDeque;
use std::ops::{Index, IndexMut};
use std::result;

pub type IntCode = isize;
pub type Program = Vec<IntCode>;

#[derive(Debug, Clone)]
pub enum CPUError {
    InvalidOpCode,
}

type Result<T> = result::Result<T, CPUError>;

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equals(Mode, Mode, Mode),
    Halt,
}

impl OpCode {
    pub fn new(int_code: IntCode) -> Result<OpCode> {
        let op_code = int_code % 100;

        let mode_one = Mode::new((int_code / 100) % 10)?;
        let mode_two = Mode::new((int_code / 1000) % 10)?;
        let mode_three = Mode::new((int_code / 10000) % 10)?;

        match op_code {
            1 => Ok(OpCode::Add(mode_one, mode_two, mode_three)),
            2 => Ok(OpCode::Mul(mode_one, mode_two, mode_three)),
            3 => Ok(OpCode::Input(mode_one)),
            4 => Ok(OpCode::Output(mode_one)),
            5 => Ok(OpCode::JumpIfTrue(mode_one, mode_two)),
            6 => Ok(OpCode::JumpIfFalse(mode_one, mode_two)),
            7 => Ok(OpCode::LessThan(mode_one, mode_two, mode_three)),
            8 => Ok(OpCode::Equals(mode_one, mode_two, mode_three)),
            99 => Ok(OpCode::Halt),
            _ => Err(CPUError::InvalidOpCode),
        }
    }

    pub fn instruction_size(&self) -> usize {
        match self {
            OpCode::Add(_, _, _) => 4,
            OpCode::Mul(_, _, _) => 4,
            OpCode::Input(_) => 2,
            OpCode::Output(_) => 2,
            OpCode::JumpIfTrue(_, _) => 3,
            OpCode::JumpIfFalse(_, _) => 3,
            OpCode::LessThan(_, _, _) => 4,
            OpCode::Equals(_, _, _) => 4,
            OpCode::Halt => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
}

impl Mode {
    pub fn new(int_code: IntCode) -> Result<Mode> {
        match int_code {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            _ => Err(CPUError::InvalidOpCode),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionState {
    Running,
    Halted,
    NeedsInput,
}

#[derive(Debug, Clone)]
pub struct Execution {
    pub ip: usize,
    pub memory: Vec<IntCode>,
    input: VecDeque<IntCode>,
    pub output: VecDeque<IntCode>,
}

impl Execution {
    pub fn new(memory: Vec<IntCode>) -> Execution {
        Self::new_input(memory, vec![])
    }

    pub fn new_input(memory: Vec<IntCode>, input: Vec<IntCode>) -> Execution {
        Execution {
            ip: 0,
            memory,
            input: input.into(),
            output: VecDeque::new(),
        }
    }

    pub fn run(&mut self) -> Result<ExecutionState> {
        log::debug!("{:?}", self);
        let mut state = self.step()?;
        while state == ExecutionState::Running {
            state = self.step()?;
            log::debug!("{:?}", self);
        }
        log::debug!("{:?}", self);

        Ok(state)
    }

    pub fn step(&mut self) -> Result<ExecutionState> {
        let op = OpCode::new(self.memory[self.ip])?;
        let mut ip_offset = op.instruction_size();

        let state = match op {
            OpCode::Add(m1, m2, m3) => {
                *self.w_off(3, m3) = self.r_off(1, m1) + self.r_off(2, m2);
                ExecutionState::Running
            }
            OpCode::Mul(m1, m2, m3) => {
                *self.w_off(3, m3) = self.r_off(1, m1) * self.r_off(2, m2);
                ExecutionState::Running
            }
            OpCode::Input(m1) => {
                let input = self.input.pop_front();

                match input {
                    Some(i) => {
                        *self.w_off(1, m1) = i;
                        ExecutionState::Running
                    }
                    None => ExecutionState::NeedsInput,
                }
            }
            OpCode::Output(m1) => {
                self.output.push_back(self.r_off(1, m1));
                ExecutionState::Running
            }
            OpCode::JumpIfTrue(m1, m2) => {
                if self.r_off(1, m1) != 0 {
                    self.ip = self.r_off(2, m2) as usize;
                    ip_offset = 0;
                }
                ExecutionState::Running
            }
            OpCode::JumpIfFalse(m1, m2) => {
                if self.r_off(1, m1) == 0 {
                    self.ip = self.r_off(2, m2) as usize;
                    ip_offset = 0;
                }
                ExecutionState::Running
            }
            OpCode::LessThan(m1, m2, m3) => {
                *self.w_off(3, m3) = if self.r_off(1, m1) < self.r_off(2, m2) {
                    1
                } else {
                    0
                };
                ExecutionState::Running
            }
            OpCode::Equals(m1, m2, m3) => {
                *self.w_off(3, m3) = if self.r_off(1, m1) == self.r_off(2, m2) {
                    1
                } else {
                    0
                };
                ExecutionState::Running
            }
            OpCode::Halt => ExecutionState::Halted,
        };

        if ExecutionState::Running == state {
            self.ip += ip_offset;
        }

        Ok(state)
    }

    fn r_off(&self, offset: IntCode, mode: Mode) -> IntCode {
        let value = self[(self.ip as isize + offset) as usize];
        match mode {
            Mode::Position => self[value as usize],
            Mode::Immediate => value,
        }
    }

    fn w_off(&mut self, offset: IntCode, mode: Mode) -> &mut IntCode {
        assert_ne!(mode, Mode::Immediate);

        let index = self[(self.ip as isize + offset) as usize];
        &mut self[index as usize]
    }
}

impl Index<usize> for Execution {
    type Output = IntCode;

    fn index(&self, address: usize) -> &Self::Output {
        &self.memory[address]
    }
}

impl IndexMut<usize> for Execution {
    fn index_mut(&mut self, address: usize) -> &mut Self::Output {
        &mut self.memory[address]
    }
}

impl From<Execution> for Vec<IntCode> {
    fn from(execution: Execution) -> Self {
        execution.memory
    }
}

impl From<Vec<IntCode>> for Execution {
    fn from(memory: Vec<IntCode>) -> Self {
        Execution::new(memory)
    }
}

pub fn parse_program(raw_memory: &str) -> Program {
    raw_memory
        .split(',')
        .map(|s| s.parse::<IntCode>().expect("parse error"))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_op_code() {
        assert_eq!(
            OpCode::Mul(Mode::Position, Mode::Immediate, Mode::Position),
            OpCode::new(1002).unwrap()
        );

        assert_eq!(
            OpCode::Mul(Mode::Position, Mode::Immediate, Mode::Immediate),
            OpCode::new(11002).unwrap()
        );
    }

    #[test]
    fn cpu_position_mode() {
        assert_eq!(vec![0], run("3,9,8,9,10,9,4,9,99,-1,8", vec![7]));
        assert_eq!(vec![1], run("3,9,8,9,10,9,4,9,99,-1,8", vec![8]));
    }

    fn run(program: &str, input: Vec<IntCode>) -> Vec<IntCode> {
        let mut execution: Execution = Execution::new_input(parse_program(program), input);

        execution.run().expect("This should always work");

        execution.output.into()
    }
}
