use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: bfi <file>");
        return;
    }

    let file_name: &str = &args[1];
    let source: String = read_source_file(file_name).unwrap();

    let instructions: Vec<char> = source.chars().collect();     // 源代码的所有指令。

    let mut mem: [u8; 30000] = [0; 30000];          // 机器的内存。
    let mut mem_ptr: usize = 0;                     // 指向内存的指针。
    let mut instruction_ptr: usize = 0;             // 指向指令的指针。

    let mut loop_begin_end_map: HashMap<usize, usize> = HashMap::new();
    let mut loop_end_begin_map: HashMap<usize, usize> = HashMap::new();

    while instruction_ptr < instructions.len() {

        let instruction = instructions[instruction_ptr];

        match instruction {
            '+' => {
                mem[mem_ptr] += 1;
                instruction_ptr += 1;
            },
            '-' => {
                mem[mem_ptr] -= 1;
                instruction_ptr += 1;
            },
            '>' => {
                mem_ptr += 1;
                instruction_ptr += 1;
            },
            '<' => {
                mem_ptr -= 1;
                instruction_ptr += 1;
            },
            '[' => {
                let loop_begin_end_ensured = loop_begin_end_map.contains_key(&instruction_ptr);
                if !loop_begin_end_ensured {
                    match find_loop_end(&instructions, instruction_ptr) {
                        Some(loop_end_ptr) => {
                            loop_begin_end_map.insert(instruction_ptr, loop_end_ptr);
                            loop_end_begin_map.insert(loop_end_ptr, instruction_ptr);
                        },
                        None => {
                            println!("ERROR: 找不到'['指令(位置：{})对应的']'指令。", instruction_ptr);
                            return;
                        }
                    }
                }
                if mem[mem_ptr] != 0 {
                    instruction_ptr += 1;
                } else {
                    instruction_ptr = loop_begin_end_map[&instruction_ptr];
                }
            },
            ']' => {
                if mem[mem_ptr] != 0 {
                    instruction_ptr = loop_end_begin_map[&instruction_ptr];
                } else {
                    instruction_ptr += 1;
                }
            },
            ',' => {
                mem[mem_ptr] = getchar();
                instruction_ptr += 1;
            },
            '.' => {
                putchar(mem[mem_ptr]);
                instruction_ptr += 1;
            },
            _ => {
                instruction_ptr += 1;
            },
        }
    }
}

fn read_source_file(file_name: &str) -> anyhow::Result<String> {

    let file: File = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

fn find_loop_end(instructions: &[char], loop_begin: usize) -> Option<usize> {
    let mut result: usize = loop_begin + 1;
    let mut layer: usize = 0;
    while result < instructions.len() {
        if instructions[result] == ']' {
            if layer == 0 {
                return Some(result);
            } else {
                layer -= 1;
            }
        }
        else if instructions[result] == '[' {
            layer += 1;
        }
        result += 1;
    }
    None
}

fn getchar() -> u8 {
    let input: Option<u8> = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok());
    input.unwrap()
}

fn putchar(byte: u8) {
    _ = std::io::stdout().write(&[byte]).unwrap();
}