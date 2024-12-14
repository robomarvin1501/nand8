use std::collections::HashMap;
use std::sync::Once;

use crate::instructions::{
    ArithmeticType, BinaryArithmeticOperator, Call, Function, Instruction, Label, Pop, Push,
    Segment, ShiftArithmeticOperator, UnaryArithmeticOperator,
};

const COMMENT_BEGIN: &str = "//";

const OPERANDS_MEMORY: [&'static str; 2] = ["push", "pop"];
const OPERANDS_GOTO: [&'static str; 2] = ["goto", "if-goto"];

const OPERAND_LABEL: &'static str = "label";

const OPERAND_CALL: &'static str = "call";

const OPERAND_FUNCTION: &'static str = "function";

const OPERAND_RETURN: &'static str = "return";

static mut CONST_HASHMAP: Option<HashMap<&'static str, Instruction>> = None;
static INIT: Once = Once::new();

fn get_const_hashmap() -> &'static HashMap<&'static str, Instruction> {
    // This code should not exist. However, the inability of the internal testing
    // system to use the internet requires a new way of making const HashMaps
    // than using an external library, and the age of the rust version on uni
    // computers (1.63 if you can believe it) means that using Lazy is not an
    // option, as that was added to std in 1.70.
    unsafe {
        INIT.call_once(|| {
            let mut map = HashMap::new();
            map.insert(
                "add",
                Instruction::CArithmetic(ArithmeticType::Binary(BinaryArithmeticOperator::Add)),
            );
            map.insert(
                "sub",
                Instruction::CArithmetic(ArithmeticType::Binary(
                    BinaryArithmeticOperator::Subtract,
                )),
            );
            map.insert(
                "neg",
                Instruction::CArithmetic(ArithmeticType::Unary(UnaryArithmeticOperator::Negate)),
            );
            map.insert(
                "and",
                Instruction::CArithmetic(ArithmeticType::Binary(BinaryArithmeticOperator::And)),
            );
            map.insert(
                "or",
                Instruction::CArithmetic(ArithmeticType::Binary(BinaryArithmeticOperator::Or)),
            );
            map.insert(
                "not",
                Instruction::CArithmetic(ArithmeticType::Unary(UnaryArithmeticOperator::Not)),
            );
            map.insert(
                "shiftleft",
                Instruction::CArithmetic(ArithmeticType::Shift(ShiftArithmeticOperator::ShiftLeft)),
            );
            map.insert(
                "shiftright",
                Instruction::CArithmetic(ArithmeticType::Shift(
                    ShiftArithmeticOperator::ShiftRight,
                )),
            );
            map.insert(
                "eq",
                Instruction::CArithmetic(ArithmeticType::Binary(BinaryArithmeticOperator::Eq)),
            );
            map.insert(
                "gt",
                Instruction::CArithmetic(ArithmeticType::Binary(BinaryArithmeticOperator::Gt)),
            );
            map.insert(
                "lt",
                Instruction::CArithmetic(ArithmeticType::Binary(BinaryArithmeticOperator::Lt)),
            );
            CONST_HASHMAP = Some(map);
        });
        CONST_HASHMAP.as_ref().unwrap()
    }
}

pub fn parse(lines: Vec<String>, function_calls: &mut HashMap<String, u16>) -> Vec<Instruction> {
    let operands_arithmetic_implicit = get_const_hashmap();
    let whitespace_cleaned_lines = clear_whitespace(lines);

    let mut current_function: String = String::new();

    let mut parsed_lines: Vec<Instruction> = vec![];
    for line in whitespace_cleaned_lines {
        // Arithmetic
        match operands_arithmetic_implicit.get(line.as_str()) {
            Some(instr) => {
                parsed_lines.push(instr.to_owned());
                continue;
            }
            None => {}
        }

        // Memory operations
        match operand_memory(&line) {
            Some(instruction) => {
                parsed_lines.push(instruction);
                continue;
            }
            None => {}
        }

        // Labels
        if line.starts_with(OPERAND_LABEL) {
            parsed_lines.push(Instruction::CLabel(Label::new(
                &current_function,
                &line.split_whitespace().nth(1).unwrap().to_string(),
            )));
            continue;
        }

        // Jumps
        match operand_gotos(&line, &current_function) {
            Some(instruction) => {
                parsed_lines.push(instruction);
                continue;
            }
            None => {}
        }

        // Call a function
        if line.starts_with(OPERAND_CALL) {
            let mut details = line.split_whitespace();
            details.next();
            let function_name: String = details.next().unwrap().to_string();
            let n_args: u16 = details.next().unwrap().parse().unwrap();

            current_function = function_name.clone();
            let n_calls: u16 = match function_calls.get(&current_function) {
                Some(e) => *e,
                None => 1,
            };
            parsed_lines.push(Instruction::CCall(Call::new(
                &function_name,
                &"FUNCTION_NAME&ret.CALL_NUMBER"
                    .replace("FUNCTION_NAME", &function_name)
                    .replace("CALL_NUMBER", &n_calls.to_string()),
                n_args,
            )));
            function_calls.insert(
                current_function.clone(),
                match n_calls {
                    1 => 2,
                    _ => n_calls + 1,
                },
            );
            continue;
        }

        // Create a function
        match operand_function(&line, &mut current_function) {
            Some(instruction) => {
                parsed_lines.push(instruction);
                continue;
            }
            None => {}
        }

        // return
        if line.starts_with(OPERAND_RETURN) {
            parsed_lines.push(Instruction::CReturn);
            continue;
        }
    }
    parsed_lines
}

fn clear_whitespace(lines: Vec<String>) -> Vec<String> {
    let mut whitespace_cleaned_lines: Vec<String> = vec![];
    for line in lines {
        if line.is_empty() || line.starts_with(COMMENT_BEGIN) {
        } else if let Some(comment_index) = line.find(COMMENT_BEGIN) {
            let trimmed = &line[..comment_index].trim();
            whitespace_cleaned_lines.push(trimmed.to_string());
        } else {
            whitespace_cleaned_lines.push(line);
        }
    }
    whitespace_cleaned_lines
}

fn operand_memory(line: &String) -> Option<Instruction> {
    for operand in OPERANDS_MEMORY {
        if !line.starts_with(operand) {
            continue;
        }
        let mut line_details = line.split_whitespace();
        line_details.next();
        let unparsed_segment = line_details.next().unwrap();
        let unparsed_index = line_details.next().unwrap();

        let segment: Segment = Segment::from(unparsed_segment);
        let index: u16 = unparsed_index.parse().unwrap();
        match operand {
            "push" => {
                return Some(Instruction::CPush(Push::new(segment, index)));
            }
            "pop" => {
                return Some(Instruction::CPop(Pop::new(segment, index)));
            }
            _ => panic!("Undefined operand: {operand}"),
        }
    }
    None
}

fn operand_gotos(line: &String, current_function: &String) -> Option<Instruction> {
    for operand in OPERANDS_GOTO {
        if !line.starts_with(operand) {
            continue;
        }
        let mut line_details = line.split_whitespace();
        let target = line_details.nth(1).unwrap();
        return match operand {
            "goto" => Some(Instruction::CGoto(Label::new(
                current_function,
                &target.to_string(),
            ))),
            "if-goto" => Some(Instruction::CIf(Label::new(
                current_function,
                &target.to_string(),
            ))),
            _ => panic!("Invalid operand {}", operand),
        };
    }
    None
}

fn operand_function(line: &String, current_function: &mut String) -> Option<Instruction> {
    if !line.starts_with(OPERAND_FUNCTION) {
        return None;
    }
    let mut line_details = line.split_whitespace();
    line_details.next(); // Remove function keyword
    *current_function = line_details.next().unwrap().to_string();
    let n_args: u16 = line_details.next().unwrap().parse().unwrap();

    Some(Instruction::CFunction(Function::new(
        &current_function,
        n_args,
    )))
}
