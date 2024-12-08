use core::panic;

use crate::instructions::{
    ArithmeticType, BinaryArithmeticOperator, Instruction, Segment, ShiftArithmeticOperator,
    UnaryArithmeticOperator,
};

const COMMAND_BINARY: &'static str = "@SP   // BINARY command
AM=M-1         // Decrement SP and point to the topmost value
D=M            // Store the topmost value (y) in D
@SP
AM=M-1         // Decrement SP again to point to the second topmost value
M=M{}D         // Perform binary operation: x OPERATOR y, store the result in the current top of stack
@SP
M=M+1          // Increment SP to point to the new top of the stack
";

const COMMAND_UNARY: &'static str = r#"@SP  // UNARY command
AM=M-1
M={}M
@SP
M=M+1
"#;

const COMMAND_SHIFT: &'static str = r#"@SP  // SHIFT command
AM=M-1
M=M{}
@SP
M=M+1
"#;

const COMMAND_COMPARE: &'static str = "@SP   // COMPARISON command (EQ, GT, LT)
AM=M-1         // Decrement SP and point to the topmost value (y)
D=M            // Store the topmost value (y) in D
@SP
AM=M-1         // Decrement SP again to point to the second topmost value (x)
D=M-D          // Subtract y from x (D = x - y)
@TRUE_COMPARE  // Jump to TRUE_COMPARE if the condition is met
D;JUMP_TYPE    // Conditional jump: JUMP_TYPE is replaced with JEQ, JGT, or JLT
@SP
A=M            // Point to the current top of the stack
M=0            // Set the result to false (0) because the condition is not met
@END_COMPARE   // Jump to END_COMPARE to skip the true case
0;JMP          // Unconditional jump to END_COMPARE
(TRUE_COMPARE)
@SP
A=M            // Point to the current top of the stack
M=-1           // Set the result to true (-1) because the condition is met
(END_COMPARE)
@SP
M=M+1          // Increment SP to point to the new top of the stack
";

const COMMAND_PUSH: &'static str = "@BASE   // PUSH command
D=SEGMENT_ACCESS // Load the base address or constant value into D
@INDEX
A=D+A            // Compute the effective address (base + index)
D=M              // Load the value at the effective address into D
@SP
A=M              // Point to the top of the stack
M=D              // Push the value onto the stack
@SP
M=M+1            // Increment the stack pointer
";

const COMMAND_PUSH_POINTER: &'static str = "@BASE // PUSH POINTER command
D=M
@SP
A=M              // Point to the top of the stack
M=D              // Push the value onto the stack
@SP
M=M+1            // Increment the stack pointer
";


const COMMAND_PUSH_CONSTANT: &'static str = "@INDEX // PUSH CONSTANT command
D=A              // Load the constant value into D
@SP
A=M              // Point to the top of the stack
M=D              // Push the constant onto the stack
@SP
M=M+1            // Increment the stack pointer
";

const COMMAND_PUSH_STATIC: &'static str = "@INDEX // PUSH STATIC command
D=M              // Load the value into D
@SP
A=M              // Point to the top of the stack
M=D              // Push the constant onto the stack
@SP
M=M+1            // Increment the stack pointer
";

const COMMAND_POP: &'static str = "@BASE    // POP command
D=SEGMENT_ACCESS // Load the base address into D
@INDEX
D=D+A            // Compute the effective address (base + index)
@R13
M=D              // Store the effective address in R13
@SP
AM=M-1           // Decrement SP and point to the topmost value
D=M              // Store the topmost value in D
@R13
A=M              // Point to the effective address
M=D              // Store the value at the effective address
";

const COMMAND_POP_DIRECT: &'static str = "@SP   // POP CONSTANT command
AM=M-1           // Decrement SP and point to the topmost value
D=M              // Store the topmost value in D
@BASE
M=D              // Store the value directly into the base address
";

const COMMAND_POP_STATIC: &'static str = "@SP // POP STATIC command
AM=M-1           // Decrement SP and point to the topmost value
D=M              // Store the topmost value in D
@INDEX
M=D              // Store the value directly in the address of the variable
";

pub fn compile(instructions: Vec<Instruction>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut comparison_count: u16 = 0;
    for instruction in instructions {
        let compiled_instruction: Option<String> = match instruction {
            Instruction::CArithmetic(number_of_operands) => Some(create_arithmetic_operator(
                number_of_operands,
                &mut comparison_count,
            )),
            Instruction::CPush(push) => {
                let asm = match push.segment {
                    Segment::Local => COMMAND_PUSH
                        .replace("BASE", "LCL")
                        .replace("SEGMENT_ACCESS", "M")
                        .replace("INDEX", &push.index.to_string()),
                    Segment::Argument => COMMAND_PUSH
                        .replace("BASE", "ARG")
                        .replace("SEGMENT_ACCESS", "M")
                        .replace("INDEX", &push.index.to_string()),
                    Segment::This => COMMAND_PUSH
                        .replace("BASE", "THIS")
                        .replace("SEGMENT_ACCESS", "M")
                        .replace("INDEX", &push.index.to_string()),
                    Segment::That => COMMAND_PUSH
                        .replace("BASE", "THAT")
                        .replace("SEGMENT_ACCESS", "M")
                        .replace("INDEX", &push.index.to_string()),
                    Segment::Pointer => {
                        let pointer_register = if push.index == 0 { "THIS" } else { "THAT" };
                        COMMAND_PUSH_POINTER
                            .replace("BASE", pointer_register)
                    }
                    Segment::Temp => COMMAND_PUSH
                        .replace("BASE", &(5 + push.index).to_string())
                        .replace("SEGMENT_ACCESS", "A")
                        .replace("INDEX", "0"), // Temp base starts at 5
                    Segment::Constant => {
                        COMMAND_PUSH_CONSTANT.replace("INDEX", &push.index.to_string())
                    }
                    Segment::Static => COMMAND_PUSH_STATIC
                        .replace("INDEX", &format!("Static.{}", push.index))
                };
                Some(asm)
            }
            Instruction::CPop(pop) => {
                let asm = match pop.segment {
                    Segment::Local => COMMAND_POP
                        .replace("BASE", "LCL")
                        .replace("SEGMENT_ACCESS", "M")
                        .replace("INDEX", &pop.index.to_string()),
                    Segment::Argument => COMMAND_POP
                        .replace("BASE", "ARG")
                        .replace("SEGMENT_ACCESS", "M")
                        .replace("INDEX", &pop.index.to_string()),
                    Segment::This => COMMAND_POP
                        .replace("BASE", "THIS")
                        .replace("SEGMENT_ACCESS", "M")
                        .replace("INDEX", &pop.index.to_string()),
                    Segment::That => COMMAND_POP
                        .replace("BASE", "THAT")
                        .replace("SEGMENT_ACCESS", "M")
                        .replace("INDEX", &pop.index.to_string()),
                    Segment::Pointer => {
                        let pointer_register = if pop.index == 0 { "THIS" } else { "THAT" };
                        COMMAND_POP_DIRECT.replace("BASE", pointer_register)
                    }
                    Segment::Temp => COMMAND_POP
                        .replace("BASE", &(5 + pop.index).to_string())
                        .replace("SEGMENT_ACCESS", "A")
                        .replace("INDEX", "0"),
                    Segment::Static => COMMAND_POP_STATIC
                        .replace("INDEX", &format!("Static.{}", pop.index)),
                    Segment::Constant => panic!("Cannot pop from constant"),
                };
                Some(asm)
            }
            _ => None,
        };
        match compiled_instruction {
            Some(instruction_asm) => result.push(instruction_asm),
            None => panic!("Couldn't compile instruction {:?}", instruction),
        }
    }
    result
}

fn create_arithmetic_operator(
    arithmetic_operator: ArithmeticType,
    comparison_count: &mut u16,
) -> String {
    match arithmetic_operator {
        ArithmeticType::Unary(operator) => match operator {
            UnaryArithmeticOperator::Negate => COMMAND_UNARY.replace("{}", "-"),
            UnaryArithmeticOperator::Not => COMMAND_UNARY.replace("{}", "!"),
        },
        ArithmeticType::Binary(operator) => match operator {
            BinaryArithmeticOperator::Add => COMMAND_BINARY.replace("{}", "+"),
            BinaryArithmeticOperator::Subtract => COMMAND_BINARY.replace("{}", "-"),
            BinaryArithmeticOperator::And => COMMAND_BINARY.replace("{}", "&"),
            BinaryArithmeticOperator::Or => COMMAND_BINARY.replace("{}", "|"),
            BinaryArithmeticOperator::Gt => {
                *comparison_count += 1;
                COMMAND_COMPARE
                    .replace("JUMP_TYPE", "JGT")
                    .replace(
                        "TRUE_COMPARE",
                        &format!("TRUE_COMPARE{}", *comparison_count),
                    )
                    .replace("END_COMPARE", &format!("END_COMPARE{}", *comparison_count))
            }
            BinaryArithmeticOperator::Eq => {
                *comparison_count += 1;
                COMMAND_COMPARE
                    .replace("JUMP_TYPE", "JEQ")
                    .replace(
                        "TRUE_COMPARE",
                        &format!("TRUE_COMPARE{}", *comparison_count),
                    )
                    .replace("END_COMPARE", &format!("END_COMPARE{}", *comparison_count))
            }
            BinaryArithmeticOperator::Lt => {
                *comparison_count += 1;
                COMMAND_COMPARE
                    .replace("JUMP_TYPE", "JLT")
                    .replace(
                        "TRUE_COMPARE",
                        &format!("TRUE_COMPARE{}", *comparison_count),
                    )
                    .replace("END_COMPARE", &format!("END_COMPARE{}", *comparison_count))
            }
        },
        ArithmeticType::Shift(operator) => match operator {
            ShiftArithmeticOperator::ShiftLeft => COMMAND_SHIFT.replace("{}", "<<"),
            ShiftArithmeticOperator::ShiftRight => COMMAND_SHIFT.replace("{}", ">>"),
        },
    }
}
