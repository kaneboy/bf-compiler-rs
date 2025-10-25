use std::io::{Read, Write};

use crate::instruction::Instruction;
use anyhow::Result;

pub struct Machine {
    instructions    : Vec<Instruction>,
    instruction_ptr : usize,
    mem             : [u8; 30000],
    mem_ptr         : usize,
}

impl Machine {

    pub fn new(instructions: Vec<Instruction>) -> Machine {
        Machine {
            instructions,
            instruction_ptr : 0,
            mem             : [0; 30000],
            mem_ptr         : 0,
        }
    }

    /// 执行下一条指令。
    ///
    ///   返回 None 表示已经没有剩余可以执行的指令。
    ///
    pub fn run_next(&mut self) -> Result<Option<()>> {

        if self.instruction_ptr == self.instructions.len() {
            return Ok(None);
        }

        let instruction = &self.instructions[self.instruction_ptr];

        match instruction {

            Instruction::Inc(val) => {
                self.mem[self.mem_ptr] += *val as u8;
                self.instruction_ptr += 1;
            },

            Instruction::Dec(val) => {
                self.mem[self.mem_ptr] -= *val as u8;
                self.instruction_ptr += 1;
            },

            Instruction::IncMemPtr(val) => {
                self.mem_ptr += *val;
                self.instruction_ptr += 1;
            },

            Instruction::DecMemPtr(val) => {
                self.mem_ptr -= *val;
                self.instruction_ptr += 1;
            },

            Instruction::GetChar(count) => {
                for _ in 0..*count {
                    self.mem[self.mem_ptr] = getchar();
                }
                self.instruction_ptr += 1;
            },

            Instruction::PutChar(count) => {
                for _ in 0..*count {
                    putchar(self.mem[self.mem_ptr]);
                }
                self.instruction_ptr += 1;
            },

            Instruction::JmpToIfZero(target) => {
                self.instruction_ptr = if self.mem[self.mem_ptr] == 0 {
                                           *target
                                       } else {
                                           self.instruction_ptr + 1
                                       };
            },

            Instruction::JmpBackIfNonZero(target) => {
                self.instruction_ptr = if self.mem[self.mem_ptr] != 0 {
                                           *target
                                       } else {
                                           self.instruction_ptr + 1
                                       };
            },
        }

        Ok(Some(()))
    }

    /// 执行所有指令。
    pub fn run_to_end(&mut self) -> Result<()> {
        while (self.run_next()?).is_some() {}
        Ok(())
    }

}

fn getchar() -> u8 {
    std::io::stdin()
        .bytes()
        .next()
        .unwrap()
        .unwrap()
}

fn putchar(byte: u8) {
    _ = std::io::stdout().write(&[byte]).unwrap();
}
