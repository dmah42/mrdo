# Assembly instructions

## HLT
Halts the current program.

## LOAD
Loads values into registers. There are variants for different register types:

*LOAD $rI #i*
Loads an integer (*i*) into an integer register (*I*).

*LOAD $rR #r*
Loads a real (*r*) into a real register (*R*).

*LOAD $rV #addr #l*
Loads a vector from memory (offset *addr*, length *l*) into a vector register (*V*).

## LW
*LW $rI $rA*

Loads a word from memory at offset stored in register *A* to an integer register *I*.

## SW
*SW $rA $rI*

Stores a word from register *I* into memory at offset stored in register *A*.

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