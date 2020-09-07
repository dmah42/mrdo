![Rust](https://github.com/dominichamon/mrdo/workflows/Rust/badge.svg)

# mrdo
mrdo is a language in which variables may be either a:

* real (mutable 64-bit floating point)
* TODO: coll (an immutable collection of reals)
* TODO: seq (an ordered immutable collection of reals)
* TODO: dict (key-value immutable collection of reals)

## usage
```bash
$ ./mrdo
```

will start a repl (that currently only understands assembly).

```bash
$ ./mrdo <filename> [-o <output>]
```

will compile the program provided and run it, optionally outputting the
bytecode. If the provided program is already bytecode, it will be run directly.

## submodules
**compiler** compiles from high-level to assembly

**asm** compiles from assembly to bytecode

**vm** runs the bytecode

**repl** understands _assembly_ at this point and (TODO) will understand high
level code later.

## laungage features

### functions

TODO

Functions operate on collections in parallel and are either a:

* `map` (convert each element in the input collection to one element in the
  output collection)
* `filter` (conditionally output each element in the input collection)
* `fold` (accumulate a collection to a single element)

For `map`, the input and output collections may be different types.

More function types may be added later.

### operations
The usual operations are available within a function for elements of a
collection (reals):

* arithmetical: +, -, /, *
* TODO: comparitive: gt, ge, lt, le, eq, ne
* TODO: logical: and, or, not, xor

Logical operations treat 0.0 as false and all other values as true.

Collections themselves have the following operations defined:

* TODO: flatten: takes multiple collections and combines them into a single
collection.
* TODO: sort: takes a collection and returns a seq of the elements in an order
governed by a given comparison operation.
* TODO: first: returns the first 'n' elements of a collection in the same collection
type.

### io
TODO:
Collections can be read or written using the functions

* read
* write

which used stdin and stdout

old
--
There's an original version of this project that is more feature rich and uses
LLVM to compile to a binary, but that restricted what could be done at runtime
so it's been moved to `old`.
