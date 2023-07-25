# Opcodes

This document lists out all the assembly language operations with arguments and
examples.

## halt (HLT)
Halts the program.

### Arguments
None

### Example
`halt`

## load (LOAD)
Loads a constant value into a register.

### Arguments
* register (any type)
* constant (any type)

### Example
`load $i29 #42`

### Note
No type coercion is done and it's an error to try to load the wrong
type of constant into the wrong register type.

This may change in later versions.

## copy (COPY)
Copies the contents of one register to another.

### Arguments
* a destination register (any type)
* a source register (any type)

### Example
`copy $r29 $i30`

### Note
Type coercion is performed where possible, including loss of precision
copying from real to integer.

<!--
    LW,
    SW,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    RET,
    EQ,
    NEQ,
    GT,
    LT,
    GTE,
    LTE,
    JEQ,
    AND,
    OR,
    NOT,
    ALLOC,
    SYSCALL,
    IGL = 255,
}

            "lw" => Opcode::LW,
            "sw" => Opcode::SW,
            "add" => Opcode::ADD,
            "sub" => Opcode::SUB,
            "mul" => Opcode::MUL,
            "div" => Opcode::DIV,
            "jmp" => Opcode::JMP,
            "eq" => Opcode::EQ,
            "neq" => Opcode::NEQ,
            "gt" => Opcode::GT,
            "lt" => Opcode::LT,
            "gte" => Opcode::GTE,
            "lte" => Opcode::LTE,
            "jeq" => Opcode::JEQ,
            "and" => Opcode::AND,
            "or" => Opcode::OR,
            "not" => Opcode::NOT,
            "alloc" => Opcode::ALLOC,
            "syscall" => Opcode::SYSCALL,
            _ => Opcode::IGL,
            -->