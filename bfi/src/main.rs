#![allow(dead_code)]

use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use anyhow::{Result, Context};
use crate::instruction::Instruction;
use crate::machine::Machine;

mod machine;
mod instruction;
mod token;
mod lexer;

fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: bfi <file>");
        return Ok(());
    }

    let file_name: &str = &args[1];
    let source: String = read_source_file(file_name).context("读取源文件")?;

    let instructions: Vec<Instruction> = lexer::parse_source_code(&source).context("解析源代码")?;

    // for instruction in &instructions {
    //     print!("{:?} ", instruction)
    // }

    let mut machine = Machine::new(instructions);
    machine.run_to_end().context("指令执行")?;

    Ok(())
}

fn read_source_file(file_name: &str) -> Result<String> {

    let file: File = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
