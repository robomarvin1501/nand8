pub const COMMAND_BINARY: &'static str = "@SP   // BINARY command
AM=M-1         // Decrement SP and point to the topmost value
D=M            // Store the topmost value (y) in D
@SP
AM=M-1         // Decrement SP again to point to the second topmost value
M=M{}D         // Perform binary operation: x OPERATOR y, store the result in the current top of stack
@SP
M=M+1          // Increment SP to point to the new top of the stack
";

pub const COMMAND_UNARY: &'static str = r#"@SP  // UNARY command
AM=M-1
M={}M
@SP
M=M+1
"#;

pub const COMMAND_SHIFT: &'static str = r#"@SP  // SHIFT command
AM=M-1
M=M{}
@SP
M=M+1
"#;

pub const COMMAND_COMPARE: &'static str = "@SP   // COMPARISON command (EQ, GT, LT)
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

pub const COMMAND_PUSH: &'static str = "@BASE   // PUSH command
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

pub const COMMAND_PUSH_DIRECT: &'static str = "@INDEX // PUSH CONSTANT command
D=ORIGIN         // Load the constant value into D
@SP
A=M              // Point to the top of the stack
M=D              // Push the constant onto the stack
@SP
M=M+1            // Increment the stack pointer
";

pub const COMMAND_POP: &'static str = "@BASE    // POP command
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

pub const COMMAND_POP_DIRECT: &'static str = "@SP   // POP CONSTANT command
AM=M-1           // Decrement SP and point to the topmost value
D=M              // Store the topmost value in D
@BASE
M=D              // Store the value directly into the base address
";

pub const COMMAND_LABEL: &'static str = "(LABEL_NAME)
";

pub const COMMAND_GOTO: &'static str = "@LABEL // GOTO
0;JMP
";

pub const COMMAND_IF_GOTO: &'static str = "@SP // IF-GOTO
AM=M-1
D=M
@LABEL
D;JNE            // True is any non zero value
";

pub const COMMAND_CALL: &'static str = "@RETURN_ADDRESS     // CALL FUNCTION
D=A                // Push return address
@SP
AM=M+1
A=A-1
M=D

@LCL               // Push LCL
D=M
@SP
AM=M+1
A=A-1
M=D

@ARG               // Push ARG
D=M
@SP
AM=M+1
A=A-1
M=D

@THIS              // Push THIS
D=M
@SP
AM=M+1
A=A-1
M=D

@THAT              // Push THAT
D=M
@SP
AM=M+1
A=A-1
M=D

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

(RETURN_ADDRESS)   // Declare return address label
";

pub const COMMAND_FUNCTION: &'static str = "(FUNCTION_NAME) // FUNCTION create new function
@SP                // Declare entry point and set up stack for local variabels
D=M
@LCL
M=D

@N_VARS            // Number of local variables to initialise
D=A
@FUNCTION_NAME$INIT_LOOP_END
D;JEQ               // If N_VARS is 0 skip initialisation

(FUNCTION_NAME$INIT_LOOP_START)    // Start of initialization loop
@SP                  // Access the stack pointer
A=M                  // Point to the current top of the stack
M=0                  // Initialize the current local variable to 0
@SP
M=M+1                // Increment the stack pointer

D=D-1                // Decrement the counter (D = nVars - 1 each iteration)
@FUNCTION_NAME$INIT_LOOP_START     // Repeat the loop if D != 0
D;JGT

(FUNCTION_NAME$INIT_LOOP_END)      // End of initialization loop
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

@SP                 // Reposition the return value for the caller (*ARG = pop())
A=M-1
D=M
@ARG
A=M
M=D

@ARG                // Restore SP of the caller (SP = ARG + 1)
D=M+1
@SP
M=D

@FRAME              // Restore THAT of the caller (THAT = *(FRAME - 1))
A=M-1
D=M
@THAT
M=D

@FRAME              // Restore THIS of the caller (THIS = *(FRAME - 2))
A=M-1
A=A-1
D=M
@THIS
M=D

@FRAME              // Restore ARG of the caller (ARG = *(FRAME - 3))
A=M-1
A=A-1
A=A-1
D=M
@ARG
M=D

@FRAME              // Restore LCL of the caller (LCL = *(FRAME - 4))
A=M-1
A=A-1
A=A-1
A=A-1
D=M
@LCL
M=D

@RET                // Go to the return address (goto RET)
A=M
0;JMP
";

pub const BOOTSTRAP: &'static str = "@256
D=A
@SP
M=D

@Sys.init$ret.0     // CALL FUNCTION
D=A                // Push return address
@SP
AM=M+1
A=A-1
M=D

@LCL               // Push LCL
D=M
@SP
AM=M+1
A=A-1
M=D

@ARG               // Push ARG
D=M
@SP
AM=M+1
A=A-1
M=D

@THIS              // Push THIS
D=M
@SP
AM=M+1
A=A-1
M=D

@THAT              // Push THAT
D=M
@SP
AM=M+1
A=A-1
M=D

@SP                // Reposition ARG for the callee
D=M
@5
D=D-A
@0
D=D-A
@ARG
M=D

@SP                // Set LCL to SP
D=M
@LCL
M=D

@Sys.init          // Jump to the function
0;JMP

(Sys.init$ret.0)   // Declare return address label
";
