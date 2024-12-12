use core::panic;
use std::mem::replace;

use crate::asm_templates::{
    COMMAND_BINARY, COMMAND_CALL, COMMAND_COMPARE, COMMAND_FUNCTION, COMMAND_GOTO, COMMAND_IF_GOTO,
    COMMAND_LABEL, COMMAND_POP, COMMAND_POP_DIRECT, COMMAND_PUSH, COMMAND_PUSH_DIRECT,
    COMMAND_RETURN, COMMAND_SHIFT, COMMAND_UNARY,
};

use crate::instructions::{
    ArithmeticType, BinaryArithmeticOperator, Call, Function, Instruction, Label, Pop, Push,
    Segment, ShiftArithmeticOperator, UnaryArithmeticOperator,
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
            Instruction::CPush(push) => create_push_operator(&push, file_name),
            Instruction::CPop(pop) => create_pop_operator(&pop, file_name),
            Instruction::CLabel(ref label) => create_label_operator(label),
            Instruction::CIf(ref label) => create_if_operator(label),
            Instruction::CGoto(ref label) => create_goto_operator(label),
            Instruction::CCall(ref call) => create_call_operator(call),
            Instruction::CFunction(ref function) => create_function_operator(function),
            Instruction::CReturn => create_return_operator(),
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
        &create_call_operator(&Call::new(
            &String::from("Sys.init"),
            &String::from("Sys.init&ret.0"),
            0,
        ))
        .unwrap(),
    );
    return code;
}

fn create_push_operator(push: &Push, file_name: &str) -> Option<String> {
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
            .replace("SEGMENT_ACCESS", "M") // TODO Changed from A?
            .replace("INDEX", "0"), // Temp base starts at 5
        Segment::Constant => COMMAND_PUSH_DIRECT
            .replace("ORIGIN", "A") // TODO should be M?
            .replace("INDEX", &push.index.to_string()),
        Segment::Static => COMMAND_PUSH_DIRECT
            .replace("ORIGIN", "M")
            .replace("INDEX", &format!("{}.{}", file_name, push.index)),
    };
    Some(asm)
}

fn create_pop_operator(pop: &Pop, file_name: &str) -> Option<String> {
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
        Segment::Temp => COMMAND_POP
            .replace("BASE", &(5 + pop.index).to_string())
            .replace("SEGMENT_ACCESS", "M") // TODO changed from A?
            .replace("INDEX", "0"),
        Segment::Static => {
            COMMAND_POP_DIRECT.replace("BASE", &format!("{}.{}", file_name, pop.index))
        }
        Segment::Pointer => {
            let pointer_register = if pop.index == 0 { "THIS" } else { "THAT" };
            COMMAND_POP_DIRECT.replace("BASE", pointer_register)
        }
        Segment::Constant => panic!("Cannot pop from constant"),
    };
    Some(asm)
}

fn create_label_operator(label: &Label) -> Option<String> {
    Some(COMMAND_LABEL.replace("LABEL_NAME", &label.extract_label_name()))
}

fn create_if_operator(label: &Label) -> Option<String> {
    Some(COMMAND_IF_GOTO.replace("LABEL", &label.extract_label_name()))
}

fn create_goto_operator(label: &Label) -> Option<String> {
    Some(COMMAND_GOTO.replace("LABEL", &label.extract_label_name()))
}

fn create_call_operator(call: &Call) -> Option<String> {
    Some(
        COMMAND_CALL
            .replace("RETURN_ADDRESS", &call.return_address)
            .replace("N_ARGS", &call.n_args.to_string())
            .replace("FUNCTION_NAME", &call.function_name)
            .replace(
                "PUSH_LCL",
                &create_push_operator(&Push::new(Segment::Local, 0), "").unwrap(),
            )
            .replace(
                "PUSH_ARG",
                &create_push_operator(&Push::new(Segment::Argument, 0), "").unwrap(),
            )
            .replace(
                "PUSH_THIS",
                &create_push_operator(&Push::new(Segment::This, 0), "").unwrap(),
            )
            .replace(
                "PUSH_THAT",
                &create_push_operator(&Push::new(Segment::That, 0), "").unwrap(),
            ),
    )
}

fn create_function_operator(function: &Function) -> Option<String> {
    let mut setup: String = String::new();
    for _ in 0..function.n_args {
        setup.push_str(&create_push_operator(&Push::new(Segment::Constant, 0), "").unwrap());
    }
    Some(
        COMMAND_FUNCTION
            .replace("FUNCTION_NAME", &function.function_name)
            .replace("SETUP_VARIABLES", &setup),
    )
}

fn create_return_operator() -> Option<String> {
    Some(COMMAND_RETURN.replace(
        "POP_ARG",
        &create_pop_operator(&Pop::new(Segment::Argument, 0), "").unwrap(),
    ))
}
