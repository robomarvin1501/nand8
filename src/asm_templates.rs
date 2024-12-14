pub const COMMAND_UNARY: &'static str = "@SP  // UNARY command
A=M-1
M={}M
";

//         return new StringBuilder()
//         .append("@SP\n")
//         .append("AM=M-1\n")
//         .append("D=M\n")
//         .append("A=A-1\n").toString();
pub const ARITHMETIC_FORMAT_1: &'static str = "@SP
AM=M-1
D=M
A=A-1
";

//         return new StringBuilder()
//         .append("D=M-D\n")
//         .append("@FALSE")
//         .append(mJumpNumber)
//         .append("\n")
//         .append("D;")
//         .append(strJump)
//         .append("\n@SP\n")
//         .append("A=M-1\n")
//         .append("M=-1\n")
//         .append("@CONTINUE")
//         .append(mJumpNumber)
//         .append("\n0;JMP\n")
//         .append("(FALSE")
//         .append(mJumpNumber)
//         .append(")\n")
//         .append("@SP\n")
//         .append("A=M-1\n")
//         .append("M=0\n")
//         .append("(CONTINUE")
//         .append(mJumpNumber)
//         .append(")\n").toString();
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

//          strACode = new StringBuilder()
//          .append("@")
//          .append(strSegment)
//          .append("\nD=M\n@")
//          .append(nIndex)
//          .append("\n")
//          .append("A=D+A\n")
//          .append("D=M\n")
//          .append("@SP\n")
//          .append("A=M\n")
//          .append("M=D\n")
//          .append("@SP\n")
//          .append("M=M+1\n").toString();
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

//         strAcode = new StringBuilder()
//         .append("@").append(strSegment)
//         .append("\nD=M\n")
//         .append("@SP\n")
//         .append("A=M\n")
//         .append("M=D\n")
//         .append("@SP\n")
//         .append("M=M+1\n").toString();
pub const COMMAND_PUSH_DIRECT: &'static str = "@INDEX // PUSH CONSTANT command
D=ORIGIN         // Load the constant value into D
@SP
A=M              // Point to the top of the stack
M=D              // Push the constant onto the stack
@SP
M=M+1            // Increment the stack pointer
";

//         strAcode = new StringBuilder().append("@").append(strSegment)
//         .append("\nD=M\n@")
//         .append(nIndex)
//         .append("\n")
//         .append("D=D+A\n")
//         .append("@R13\n")
//         .append("M=D\n")
//         .append("@SP\n")
//         .append("AM=M-1\n")
//         .append("D=M\n")
//         .append("@R13\n")
//         .append("A=M\n")
//         .append("M=D\n").toString();
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

//         strAcode = new StringBuilder().append("@").append(strSegment)
//         .append("\nD=A\n")
//         .append("@R13\n")
//         .append("M=D\n")
//         .append("@SP\n")
//         .append("AM=M-1\n")
//         .append("D=M\n")
//         .append("@R13\n")
//         .append("A=M\n")
//         .append("M=D\n").toString();
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

//             fw.write(new StringBuilder()
//             .append("@").append(strLabel)
//             .append("\n")
//             .append("D=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n")
//             .append(getPushFormat2("LCL"))
//             .append(getPushFormat2("ARG"))
//             .append(getPushFormat2("THIS"))
//             .append(getPushFormat2("THAT"))
//             .append("@SP\n")
//             .append("D=M\n")
//             .append("@5\n")
//             .append("D=D-A\n")
//             .append("@")
//             .append(nNumArgs)
//             .append("\n")
//             .append("D=D-A\n")
//             .append("@ARG\n")
//             .append("M=D\n")
//             .append("@SP\n")
//             .append("D=M\n")
//             .append("@LCL\n")
//             .append("M=D\n")
//             .append("@")
//             .append(strFunctionName)
//             .append("\n0;JMP\n(")
//             .append(strLabel)
//             .append(")\n").toString());
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

//             fw.write(new StringBuilder()
//             .append("@LCL\n")
//             .append("D=M\n")
//             .append("@FRAME\n")
//             .append("M=D\n")
//             .append("@5\n")
//             .append("A=D-A\n")
//             .append("D=M\n")
//             .append("@RET\n")
//             .append("M=D\n")
//             .append(getPopFormat1("ARG", 0))
//             .append("@ARG\n")
//             .append("D=M\n")
//             .append("@SP\n")
//             .append("M=D+1\n")
//
//             .append("@FRAME\n")
//             .append("D=M-1\n")
//             .append("AM=D\n")
//             .append("D=M\n")
//             .append("@THAT\n")
//             .append("M=D\n")
//
//             .append("@FRAME\n")
//             .append("D=M-1\n")
//             .append("AM=D\n")
//             .append("D=M\n")
//             .append("@THIS\n")
//             .append("M=D\n")
//
//             .append("@FRAME\n")
//             .append("D=M-1\n")
//             .append("AM=D\n")
//             .append("D=M\n")
//             .append("@ARG\n")
//             .append("M=D\n")
//
//             .append("@FRAME\n")
//             .append("D=M-1\n")
//             .append("AM=D\n")
//             .append("D=M\n")
//
//             .append("@LCL\n")
//             .append("M=D\n")
//             .append("@RET\n")
//             .append("A=M\n")
//             .append("0;JMP\n").toString());
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

