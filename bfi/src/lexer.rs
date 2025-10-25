use anyhow::Result;

use crate::{instruction::Instruction, token::Token};

/// 将源代码解析成一组 Instruction 。
pub fn parse_source_code(source_code: &str) -> Result<Vec<Instruction>> {

    // 从源代码解析出一组 token ，过滤掉无效字符。
    let tokens : Vec<Token> = source_code
        .chars()
        .filter_map(|c| Token::try_from(c).ok())
        .collect();

    // 要返回的所有指令集合。
    let mut instructions : Vec<Instruction> = vec![];

    // 在解析过程中，存放 '[' 的位置。
    // 遇到 '[' 的时候，将它出现的位置压入此堆栈。
    // 遇到 ']' 的时候，从堆栈弹出上一个 '[' 的位置。
    let mut jump_begin_pos_stack : Vec<usize> = vec![];

    for token in tokens {

        if token.0 == '+' {
            if !instructions.is_empty() &&
                let Some(last_instruction) = instructions.last_mut() &&
                let Instruction::Inc(count) = last_instruction {

                *count += 1;
                continue;
            }
            instructions.push(Instruction::Inc(1));
        }

        else if token.0 == '-' {
            if !instructions.is_empty() &&
                let Some(last_instruction) = instructions.last_mut() &&
                let Instruction::Dec(count) = last_instruction {

                *count += 1;
                continue;
            }
            instructions.push(Instruction::Dec(1));
        }

        else if token.0 == '>' {
            if !instructions.is_empty() &&
                let Some(last_instruction) = instructions.last_mut() &&
                let Instruction::IncMemPtr(count) = last_instruction {

                *count += 1;
                continue;
            }
            instructions.push(Instruction::IncMemPtr(1));
        }

        else if token.0 == '<' {
            if !instructions.is_empty()
                && let Some(last_instruction) = instructions.last_mut()
                && let Instruction::DecMemPtr(count) = last_instruction
            {
                *count += 1;
                continue;
            }
            instructions.push(Instruction::DecMemPtr(1));
        }

        else if token.0 == ',' {
            if !instructions.is_empty()
                && let Some(last_instruction) = instructions.last_mut()
                && let Instruction::GetChar(count) = last_instruction
            {
                *count += 1;
                continue;
            }
            instructions.push(Instruction::GetChar(1));
        }

        else if token.0 == '.' {
            if !instructions.is_empty()
                && let Some(last_instruction) = instructions.last_mut()
                && let Instruction::PutChar(count) = last_instruction
            {

                *count += 1;
                continue;
            }
            instructions.push(Instruction::PutChar(1));
        }

        else if token.0 == '[' {
            instructions.push(Instruction::JmpToIfZero(0));
            jump_begin_pos_stack.push(instructions.len() - 1);
        }

        else if token.0 == ']' {
            if let Some(jump_begin_pos) = jump_begin_pos_stack.pop() {

                instructions.push(Instruction::JmpBackIfNonZero(jump_begin_pos));
                let jump_end_pos = instructions.len() - 1;

                if let Some(instruction) = instructions.get_mut(jump_begin_pos)
                    && let Instruction::JmpToIfZero(pos) = instruction
                {
                    *pos = jump_end_pos;
                }

            } else {
                anyhow::bail!("源代码解析错误。");
            }
        }
    }

    Ok(instructions)
}
