use core::panic;

use crate::asm_templates::{
    COMMAND_BINARY, COMMAND_CALL, COMMAND_COMPARE, COMMAND_FUNCTION, COMMAND_GOTO, COMMAND_IF_GOTO,
    COMMAND_LABEL, COMMAND_POP, COMMAND_POP_DIRECT, COMMAND_PUSH, COMMAND_PUSH_DIRECT,
    COMMAND_RETURN, COMMAND_SHIFT, COMMAND_UNARY,
};

use crate::instructions::{
    ArithmeticType, BinaryArithmeticOperator, Instruction, Segment, ShiftArithmeticOperator,
    UnaryArithmeticOperator,
};

pub fn compile(instructions: Vec<Instruction>, file_name: &str) -> Vec<String> {
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
                        COMMAND_PUSH_DIRECT
                            .replace("ORIGIN", "M")
                            .replace("INDEX", pointer_register)
                    }
                    Segment::Temp => COMMAND_PUSH
                        .replace("BASE", &(5 + push.index).to_string())
                        .replace("SEGMENT_ACCESS", "A")
                        .replace("INDEX", "0"), // Temp base starts at 5
                    Segment::Constant => COMMAND_PUSH_DIRECT
                        .replace("ORIGIN", "A")
                        .replace("INDEX", &push.index.to_string()),
                    Segment::Static => COMMAND_PUSH_DIRECT
                        .replace("ORIGIN", "M")
                        .replace("INDEX", &format!("{}.{}", file_name, push.index)),
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
                    Segment::Static => {
                        COMMAND_POP_DIRECT.replace("BASE", &format!("{}.{}", file_name, pop.index))
                    }
                    Segment::Constant => panic!("Cannot pop from constant"),
                };
                Some(asm)
            }
            Instruction::CLabel(ref label) => {
                Some(COMMAND_LABEL.replace("LABEL_NAME", &label.extract_label_name()))
            }
            Instruction::CIf(ref label) => {
                Some(COMMAND_IF_GOTO.replace("LABEL", &label.extract_label_name()))
            }
            Instruction::CGoto(ref label) => {
                Some(COMMAND_GOTO.replace("LABEL", &label.extract_label_name()))
            }
            Instruction::CCall(ref call) => Some(
                COMMAND_CALL
                    .replace("RETURN_ADDRESS", &call.return_address)
                    .replace("N_ARGS", &call.n_args.to_string())
                    .replace("FUNCTION_NAME", &call.function_name),
            ),
            Instruction::CFunction(ref function) => Some(
                COMMAND_FUNCTION
                    .replace("FUNCTION_NAME", &function.function_name)
                    .replace("N_VARS", &function.n_args.to_string()),
            ),
            Instruction::CReturn => Some(COMMAND_RETURN.to_string()),
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

pub fn create_bootstrap_code() -> String {
    let mut code: String = String::from(
        "@256   // BOOTSTRAP - set stack pointer
D=A
@SP
M=D
",
    );
    code.push_str(
        &COMMAND_CALL
            .replace("RETURN_ADDRESS", "Sys.init&ret.0")
            .replace("N_ARGS", &0.to_string())
            .replace("FUNCTION_NAME", "Sys.init"),
    );
    return code;
}

