pub const COMMAND_UNARY: &'static str = "@SP  // UNARY command
A=M-1
M={}M
";

pub const ARITHMETIC_FORMAT_1: &'static str = "@SP
AM=M-1
D=M
A=A-1
";

pub const ARITHMETIC_FORMAT_2: &'static str = "D=M-D
@FALSE.JUMP_NUMBER
D;JUMP_TYPE
@SP
A=M-1
M=-1
@CONTINUE.JUMP_NUMBER
0;JMP

(FALSE.JUMP_NUMBER)
@SP
A=M-1
M=0

(CONTINUE.JUMP_NUMBER)
";

pub const COMMAND_SHIFT: &'static str = "@SP  // SHIFT command
AM=M-1
M=M{}
@SP
M=M+1
";

pub const COMMAND_PUSH: &'static str = "@SEGMENT   // PUSH command
D=M              // Load the base address or constant value into D
@INDEX
A=D+A            // Compute the effective address (base + index)
D=M              // Load the value at the effective address into D
@SP
A=M              // Point to the top of the stack
M=D              // Push the value onto the stack
@SP
M=M+1            // Increment the stack pointer
";

pub const COMMAND_PUSH_DIRECT: &'static str = "@INDEX // PUSH CONSTANT command
D=ORIGIN         // Load the constant value into D
@SP
A=M              // Point to the top of the stack
M=D              // Push the constant onto the stack
@SP
M=M+1            // Increment the stack pointer
";

pub const COMMAND_POP: &'static str = "@SEGMENT    // POP command
D=M              // Load the base address into D
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

pub const COMMAND_POP_DIRECT: &'static str = "@SEGMENT // POP DIRECT
D=A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
";

pub const COMMAND_LABEL: &'static str = "(LABEL_NAME)
";

pub const COMMAND_GOTO: &'static str = "@LABEL // GOTO
0;JMP
";

pub const COMMAND_IF_GOTO: &'static str = "// IF-GOTO
ARITHMETIC_FORMAT_1
@LABEL
D;JNE            // True is any non zero value
";

pub const COMMAND_CALL: &'static str = "@FUNCTION_LABEL     // CALL FUNCTION
D=A                // Push return address
@SP
AM=M+1
A=A-1
M=D

PUSH_LCL

PUSH_ARG

PUSH_THIS

PUSH_THAT

@SP                // Reposition ARG for the callee
D=M
@5
D=D-A
@N_ARGS
D=D-A
@ARG
M=D

@SP                // Set LCL to SP
D=M
@LCL
M=D

@FUNCTION_NAME          // Jump to the function
0;JMP

(FUNCTION_LABEL)   // Declare return address label
";

pub const COMMAND_FUNCTION: &'static str = "(FUNCTION_NAME) // FUNCTION create new function
SETUP_VARIABLES
";

pub const COMMAND_RETURN: &'static str = "@LCL  // COMMAND RETURN
D=M                 // Save the current LCL in a temporary variable (FRAME = LCL)
@FRAME
M=D

@5                  // Get the return address (RET = *(FRAME - 5))
A=D-A
D=M
@RET
M=D

POP_ARG

@ARG                // Restore SP of the caller (SP = ARG + 1)
D=M+1
@SP
M=D

@FRAME              // Restore THAT of the caller (THAT = *(FRAME - 1))
D=M-1
AM=D
D=M
@THAT
M=D

@FRAME              // Restore THIS of the caller (THIS = *(FRAME - 2))
D=M-1
AM=D
D=M
@THIS
M=D

@FRAME              // Restore ARG of the caller (ARG = *(FRAME - 3))
D=M-1
AM=D
D=M
@ARG
M=D

@FRAME              // Restore LCL of the caller (LCL *(FRAME - 4))
D=M-1
AM=D
D=M
@LCL
M=D

@RET                // Go to the return address (goto RET)
A=M
0;JMP
";

