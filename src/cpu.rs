use std::collections::VecDeque;
use std::ops::{Index, IndexMut};
use std::result;

pub type IntCode = isize;
pub type Memory = Vec<IntCode>;

#[derive(Debug, Clone)]
pub enum CPUError {
    InvalidOpCode,
}

type Result<T> = result::Result<T, CPUError>;

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl OpCode {
    pub fn new(instruction: IntCode) -> Result<OpCode> {
        let op_code = instruction % 100;

        match op_code {
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Mul),
            3 => Ok(OpCode::Input),
            4 => Ok(OpCode::Output),
            5 => Ok(OpCode::JumpIfTrue),
            6 => Ok(OpCode::JumpIfFalse),
            7 => Ok(OpCode::LessThan),
            8 => Ok(OpCode::Equals),
            99 => Ok(OpCode::Halt),
            _ => Err(CPUError::InvalidOpCode),
        }
    }

    pub fn instruction_size(&self) -> usize {
        match self {
            OpCode::Add | OpCode::Mul | OpCode::LessThan | OpCode::Equals => 4,
            OpCode::Input | OpCode::Output => 2,
            OpCode::JumpIfTrue | OpCode::JumpIfFalse => 3,
            OpCode::Halt => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    op_code: OpCode,
    modes: [Mode; 3],
}

impl Instruction {
    pub fn new(instruction: IntCode) -> Result<Instruction> {
        let mode_one = Mode::new((instruction / 100) % 10)?;
        let mode_two = Mode::new((instruction / 1000) % 10)?;
        let mode_three = Mode::new((instruction / 10000) % 10)?;

        Ok(Instruction {
            op_code: OpCode::new(instruction)?,
            modes: [mode_one, mode_two, mode_three],
        })
    }

    pub fn size(&self) -> usize {
        self.op_code.instruction_size()
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
    pub memory: Memory,
    pub input: VecDeque<IntCode>,
    pub output: VecDeque<IntCode>,
}

impl Execution {
    pub fn new(memory: Memory) -> Execution {
        Self::new_input(memory, vec![])
    }

    pub fn new_input(memory: Memory, input: Memory) -> Execution {
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
        let instruction = Instruction::new(self.memory[self.ip])?;
        let mut ip_offset = instruction.size();

        let Instruction {
            op_code,
            modes: parameters,
        } = instruction;

        let state = match op_code {
            OpCode::Add => {
                *parameters.w3(self) = parameters.r1(self) + parameters.r2(self);
                ExecutionState::Running
            }
            OpCode::Mul => {
                *parameters.w3(self) = parameters.r1(self) * parameters.r2(self);
                ExecutionState::Running
            }
            OpCode::Input => {
                let input = self.input.pop_front();

                match input {
                    Some(i) => {
                        *parameters.w1(self) = i;
                        ExecutionState::Running
                    }
                    None => ExecutionState::NeedsInput,
                }
            }
            OpCode::Output => {
                self.output.push_back(parameters.r1(self));
                ExecutionState::Running
            }
            OpCode::JumpIfTrue => {
                if parameters.r1(self) != 0 {
                    self.ip = parameters.r2(self) as usize;
                    ip_offset = 0;
                }
                ExecutionState::Running
            }
            OpCode::JumpIfFalse => {
                if parameters.r1(self) == 0 {
                    self.ip = parameters.r2(self) as usize;
                    ip_offset = 0;
                }
                ExecutionState::Running
            }
            OpCode::LessThan => {
                *parameters.w3(self) = if parameters.r1(self) < parameters.r2(self) {
                    1
                } else {
                    0
                };
                ExecutionState::Running
            }
            OpCode::Equals => {
                *parameters.w3(self) = if parameters.r1(self) == parameters.r2(self) {
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

impl From<Execution> for Memory {
    fn from(execution: Execution) -> Self {
        execution.memory
    }
}

impl From<Memory> for Execution {
    fn from(memory: Memory) -> Self {
        Execution::new(memory)
    }
}

trait ParameterExtractor {
    fn r1(&self, execution: &Execution) -> IntCode {
        self.read(0, execution)
    }

    fn r2(&self, execution: &Execution) -> IntCode {
        self.read(1, execution)
    }

    fn r3(&self, execution: &Execution) -> IntCode {
        self.read(2, execution)
    }

    fn w1<'a>(&self, execution: &'a mut Execution) -> &'a mut IntCode {
        self.write(0, execution)
    }

    fn w2<'a>(&self, execution: &'a mut Execution) -> &'a mut IntCode {
        self.write(1, execution)
    }

    fn w3<'a>(&self, execution: &'a mut Execution) -> &'a mut IntCode {
        self.write(2, execution)
    }

    fn read(&self, offset: isize, execution: &Execution) -> IntCode;

    fn write<'a>(&self, offset: isize, execution: &'a mut Execution) -> &'a mut IntCode;
}

impl ParameterExtractor for [Mode; 3] {
    fn read(&self, offset: isize, execution: &Execution) -> isize {
        let value = execution[(execution.ip as isize + offset + 1) as usize];
        match self[offset as usize] {
            Mode::Position => execution[value as usize],
            Mode::Immediate => value,
        }
    }

    fn write<'a>(&self, offset: isize, execution: &'a mut Execution) -> &'a mut isize {
        assert_ne!(self[offset as usize], Mode::Immediate);

        let index = execution[(execution.ip as isize + offset + 1) as usize];
        &mut execution[index as usize]
    }
}

pub fn parse_program(raw_memory: &str) -> Memory {
    raw_memory
        .split(',')
        .map(|s| s.parse::<IntCode>().expect("parse error"))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_instruction() {
        assert_eq!(
            Instruction {
                op_code: OpCode::Mul,
                modes: [Mode::Position, Mode::Immediate, Mode::Position]
            },
            Instruction::new(1002).unwrap()
        );

        assert_eq!(
            Instruction {
                op_code: OpCode::Mul,
                modes: [Mode::Position, Mode::Immediate, Mode::Immediate]
            },
            Instruction::new(11002).unwrap()
        );
    }

    #[test]
    fn cpu_position_mode() {
        assert_eq!(run("3,9,8,9,10,9,4,9,99,-1,8", vec![7]), vec![0]);
        assert_eq!(run("3,9,8,9,10,9,4,9,99,-1,8", vec![8]), vec![1]);
    }

    fn run(program: &str, input: Memory) -> Vec<IntCode> {
        let mut execution: Execution = Execution::new_input(parse_program(program), input);

        execution.run().expect("This should always work");

        execution.output.into()
    }
}
