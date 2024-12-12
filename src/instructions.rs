use core::{fmt, panic};

#[derive(Debug, Clone)]
pub enum Instruction {
    CArithmetic(ArithmeticType),
    CPush(Push),
    CPop(Pop),
    CLabel(Label),
    CGoto(Label),
    CIf(Label),
    CFunction(Function),
    CReturn,
    CCall(Call),
}

#[derive(Debug, Clone, Copy)]
pub enum ArithmeticType {
    Unary(UnaryArithmeticOperator),
    Binary(BinaryArithmeticOperator),
    Shift(ShiftArithmeticOperator),
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryArithmeticOperator {
    Negate,
    Not,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryArithmeticOperator {
    Add,
    Subtract,
    And,
    Or,
    Eq,
    Gt,
    Lt,
}

#[derive(Debug, Clone, Copy)]
pub enum ShiftArithmeticOperator {
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone, Copy)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

impl Segment {
    pub fn from(segment: &str) -> Segment {
        match segment.to_lowercase().as_str() {
            "argument" => Segment::Argument,
            "local" => Segment::Local,
            "static" => Segment::Static,
            "constant" => Segment::Constant,
            "this" => Segment::This,
            "that" => Segment::That,
            "pointer" => Segment::Pointer,
            "temp" => Segment::Temp,
            _ => panic!("Not a valid segment"),
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Segment::Argument => write!(f, "ARG"),
            Segment::Local => write!(f, "LCL"),
            Segment::This => write!(f, "THIS"),
            Segment::That => write!(f, "THAT"),
            Segment::Temp => write!(f, "TEMP"),
            Segment::Static => todo!(),
            Segment::Constant => todo!(),
            Segment::Pointer => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Push {
    pub segment: Segment,
    pub index: u16,
}

impl Push {
    pub fn new(segment: Segment, index: u16) -> Self {
        Self { segment, index }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pop {
    pub segment: Segment,
    pub index: u16,
}

impl Pop {
    pub fn new(segment: Segment, index: u16) -> Self {
        Self { segment, index }
    }
}

#[derive(Debug, Clone)]
pub struct Label {
    pub parent_function: String,
    pub label: String,
}

impl Label {
    pub fn new(parent_function: &String, label: &String) -> Self {
        Self {
            parent_function: parent_function.to_string(),
            label: label.to_string(),
        }
    }
    pub fn extract_label_name(&self) -> String {
        let parent = if self.parent_function.is_empty() {
            "null"
        } else {
            &self.parent_function
        };
        "FUNCTION_NAME$LABEL"
            .replace("FUNCTION_NAME", parent)
            .replace("LABEL", &self.label)
    }
}

#[derive(Debug, Clone)]
pub struct Call {
    pub function_name: String,
    pub return_address: String,
    pub n_args: u16,
}

impl Call {
    pub fn new(function_name: &String, return_address: &String, n_args: u16) -> Self {
        Self {
            function_name: function_name.to_string(),
            return_address: return_address.to_string(),
            n_args,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub function_name: String,
    pub n_args: u16,
}

impl Function {
    pub fn new(function_name: &String, n_args: u16) -> Self {
        Self {
            function_name: function_name.to_string(),
            n_args
        }
    }
}
