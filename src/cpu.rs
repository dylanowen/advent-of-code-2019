use std::ops::{Index, IndexMut};
use std::result;

pub type IntCode = usize;
pub type Program = Vec<IntCode>;

#[derive(Debug, Clone)]
pub enum CPUError {
    InvalidOpCode,
}

type Result<T> = result::Result<T, CPUError>;

#[derive(Debug, Clone)]
pub enum OpCode {
    Add,
    Mul,
    Halt,
}

impl OpCode {
    pub fn new(int_code: IntCode) -> Option<OpCode> {
        match int_code {
            1 => Some(OpCode::Add),
            2 => Some(OpCode::Mul),
            99 => Some(OpCode::Halt),
            _ => None,
        }
    }

    pub fn instruction_size(&self) -> usize {
        4
    }
}

#[derive(Debug, Clone)]
pub struct Execution {
    pub ip: usize,
    pub memory: Vec<IntCode>,
}

impl Execution {
    pub fn new(memory: Vec<IntCode>) -> Execution {
        Execution { ip: 0, memory }
    }

    pub fn run(&mut self) -> Result<()> {
        log::debug!("{:?}", self);
        while !self.step()? {
            log::debug!("{:?}", self);
        }
        log::debug!("{:?}", self);

        Ok(())
    }

    pub fn step(&mut self) -> Result<bool> {
        let op = match OpCode::new(self.memory[self.ip]) {
            Some(op) => Ok(op),
            None => Err(CPUError::InvalidOpCode),
        }?;

        let halt = match op {
            OpCode::Add => {
                *self.w_off(3) = self.r_off(1) + self.r_off(2);
                false
            }
            OpCode::Mul => {
                *self.w_off(3) = self.r_off(1) * self.r_off(2);
                false
            }
            OpCode::Halt => true,
        };

        if !halt {
            self.ip += op.instruction_size();
        }

        Ok(halt)
    }

    fn r_off(&self, offset: IntCode) -> IntCode {
        let index = self[self.ip + offset];
        self[index]
    }

    fn w_off(&mut self, offset: IntCode) -> &mut IntCode {
        let index = self[self.ip + offset];
        &mut self[index]
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
    fn from(cpu: Execution) -> Self {
        cpu.memory
    }
}

impl From<Vec<IntCode>> for Execution {
    fn from(memory: Vec<usize>) -> Self {
        Execution::new(memory)
    }
}

pub fn parse_program(raw_memory: &str) -> Program {
    raw_memory
        .split(',')
        .map(|s| s.parse::<usize>().expect("parse error"))
        .collect()
}
