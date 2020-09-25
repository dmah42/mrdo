# Assembly instructions

## HLT
Halts the current program.

## LOAD
Loads values into registers. There are variants for different register types:

*LOAD $rI #i*
Loads an integer (*i*) into an integer register (*I*).

*LOAD $rR #r*
Loads a real (*r*) into a real register (*R*).

*LOAD $rV $iA #l*
Loads a vector from memory (offset in integer register *A*, length *l*) into a vector register (*V*).

## LW
*LW $rI $rA*

Loads a word from memory at offset stored in register *A* to an integer register *I*.

## SW
(* TODO: Change the name of this instruction...)

*SW $iA $iI*

Stores a word from register *I* into memory at offset stored in register *A*.

*SW $iA $rR*

Stores a dword (8 bytes) from real register *R* into memory at offset stored in register *A*.

(* TODO *)
## ADD,
## SUB,
## MUL,
## DIV,
## JMP,
## EQ,
## NEQ,
## GT,
## LT,
## GTE,
## LTE,
## JEQ,
## ALLOC,
## PRINT,