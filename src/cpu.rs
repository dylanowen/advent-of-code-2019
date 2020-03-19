use std::collections::VecDeque;
use std::ops::{Index, IndexMut};
use std::result;
use wasm_bindgen::prelude::*;

pub type IntCode = i64;
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
    AdjustBase,
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
            9 => Ok(OpCode::AdjustBase),
            99 => Ok(OpCode::Halt),
            _ => Err(CPUError::InvalidOpCode),
        }
    }

    pub fn instruction_size(&self) -> usize {
        match self {
            OpCode::Add | OpCode::Mul | OpCode::LessThan | OpCode::Equals => 4,
            OpCode::Input | OpCode::Output | OpCode::AdjustBase => 2,
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
    Relative,
}

impl Mode {
    pub fn new(int_code: IntCode) -> Result<Mode> {
        match int_code {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            _ => Err(CPUError::InvalidOpCode),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionState {
    Running,
    Halted,
    NeedsInput,
}

#[derive(Debug, Clone)]
pub struct Execution {
    pub ip: usize,
    pub relative_base: usize,
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
            relative_base: 0,
            memory,
            input: input.into(),
            output: VecDeque::new(),
        }
    }

    pub fn run(&mut self) -> Result<ExecutionState> {
        let mut state = self.step()?;
        while state == ExecutionState::Running {
            state = self.step()?;
        }

        Ok(state)
    }

    pub fn step(&mut self) -> Result<ExecutionState> {
        let instruction = Instruction::new(self.memory[self.ip])?;
        let mut ip_offset = instruction.size();

        let (op_code, parms) = self.prepare_instruction(instruction);
        let state = match op_code {
            OpCode::Add => {
                *parms.w(3) = parms.r(1) + parms.r(2);
                ExecutionState::Running
            }
            OpCode::Mul => {
                *parms.w(3) = parms.r(1) * parms.r(2);
                ExecutionState::Running
            }
            OpCode::Input => {
                let input = self.input.pop_front();

                match input {
                    Some(i) => {
                        *parms.w(1) = i;
                        ExecutionState::Running
                    }
                    None => ExecutionState::NeedsInput,
                }
            }
            OpCode::Output => {
                self.output.push_back(parms.r(1));
                ExecutionState::Running
            }
            OpCode::JumpIfTrue => {
                if parms.r(1) != 0 {
                    self.ip = parms.r(2) as usize;
                    ip_offset = 0;
                }
                ExecutionState::Running
            }
            OpCode::JumpIfFalse => {
                if parms.r(1) == 0 {
                    self.ip = parms.r(2) as usize;
                    ip_offset = 0;
                }
                ExecutionState::Running
            }
            OpCode::LessThan => {
                *parms.w(3) = if parms.r(1) < parms.r(2) { 1 } else { 0 };
                ExecutionState::Running
            }
            OpCode::Equals => {
                *parms.w(3) = if parms.r(1) == parms.r(2) { 1 } else { 0 };
                ExecutionState::Running
            }
            OpCode::AdjustBase => {
                self.relative_base = ((self.relative_base as IntCode) + parms.r(1)) as usize;

                ExecutionState::Running
            }
            OpCode::Halt => ExecutionState::Halted,
        };

        if ExecutionState::Running == state {
            self.ip += ip_offset;
        }

        Ok(state)
    }

    fn prepare_instruction(&mut self, instruction: Instruction) -> (OpCode, ParameterExtractor) {
        let Instruction { op_code, modes } = instruction;

        (
            op_code,
            ParameterExtractor {
                execution: self,
                modes,
            },
        )
    }

    pub fn expect_pop(&mut self) -> IntCode {
        self.output
            .pop_front()
            .expect("Expected an output from our program")
    }
}

impl Index<usize> for Execution {
    type Output = IntCode;

    fn index(&self, address: usize) -> &Self::Output {
        if address >= self.memory.len() {
            // memory is initialized to zero
            &0
        } else {
            &self.memory[address]
        }
    }
}

impl IndexMut<usize> for Execution {
    fn index_mut(&mut self, address: usize) -> &mut Self::Output {
        if address >= self.memory.len() {
            let mut to_append = vec![0; address - self.memory.len() + 1];
            self.memory.append(&mut to_append);
        }

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

struct ParameterExtractor<'a> {
    execution: &'a mut Execution,
    modes: [Mode; 3],
}

impl<'a> ParameterExtractor<'a> {
    fn r(&self, offset: isize) -> IntCode {
        let value = self.execution[(self.execution.ip as isize + offset + 1) as usize];
        match self.modes[offset as usize] {
            Mode::Position => self.execution[value as usize],
            Mode::Immediate => value,
            Mode::Relative => {
                let address = (self.execution.relative_base as IntCode) + value;
                self.execution[address as usize]
            }
        }
    }

    fn w(self, offset: IntCode) -> &'a mut IntCode {
        let value = self.execution[(self.execution.ip as IntCode + offset + 1) as usize];
        match self.modes[offset as usize] {
            Mode::Position => &mut self.execution[value as usize],
            Mode::Immediate => panic!("We should never write in immediate mode"),
            Mode::Relative => {
                let address = (self.execution.relative_base as IntCode) + value;
                &mut self.execution[address as usize]
            }
        }
    }
}

trait ParameterExtractorld {
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

    fn read(&self, offset: IntCode, execution: &Execution) -> IntCode;

    fn write<'a>(&self, offset: IntCode, execution: &'a mut Execution) -> &'a mut IntCode;
}

impl ParameterExtractorld for [Mode; 3] {
    fn read(&self, offset: IntCode, execution: &Execution) -> IntCode {
        let value = execution[(execution.ip as IntCode + offset + 1) as usize];
        match self[offset as usize] {
            Mode::Position => execution[value as usize],
            Mode::Immediate => value,
            Mode::Relative => {
                let address = (execution.relative_base as IntCode) + value;
                execution[address as usize]
            }
        }
    }

    fn write<'a>(&self, offset: IntCode, execution: &'a mut Execution) -> &'a mut IntCode {
        let value = execution[(execution.ip as IntCode + offset + 1) as usize];
        match self[offset as usize] {
            Mode::Position => &mut execution[value as usize],
            Mode::Immediate => panic!("We should never write in immediate mode"),
            Mode::Relative => {
                let address = (execution.relative_base as IntCode) + value;
                &mut execution[address as usize]
            }
        }
    }
}

pub fn parse_program(raw_memory: &str) -> Memory {
    raw_memory
        .split(',')
        .map(|s| {
            let trimmed = s.trim();
            trimmed.parse::<IntCode>().unwrap_or_else(|_| {
                panic!("Parse Error: {} couldn't be parsed as IntCode", trimmed)
            })
        })
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
                modes: [Mode::Position, Mode::Immediate, Mode::Position],
            },
            Instruction::new(1002).unwrap()
        );

        assert_eq!(
            Instruction {
                op_code: OpCode::Mul,
                modes: [Mode::Position, Mode::Immediate, Mode::Immediate],
            },
            Instruction::new(11002).unwrap()
        );
    }

    #[test]
    fn cpu_position_mode() {
        assert_eq!(run("3,9,8,9,10,9,4,9,99,-1,8", vec![7]), vec![0]);
        assert_eq!(run("3,9,8,9,10,9,4,9,99,-1,8", vec![8]), vec![1]);
    }

    #[test]
    fn cpu_relative_mode() {
        assert_eq!(
            run(
                "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
                vec![],
            ),
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
        assert_eq!(
            run("1102,34915192,34915192,7,4,7,99,0", vec![]),
            vec![1219070632396864]
        );
        assert_eq!(
            run("104,1125899906842624,99", vec![]),
            vec![1125899906842624]
        );
    }

    fn run(program: &str, input: Memory) -> Vec<IntCode> {
        let mut execution: Execution = Execution::new_input(parse_program(program), input);

        execution.run().expect("This should always work");

        execution.output.into()
    }
}
