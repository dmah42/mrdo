[![Rust](https://github.com/dmah42/mrdo/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/dmah42/mrdo/actions/workflows/rust.yml)
# mrdo
mrdo is a language in which variables may be either a:
* real (mutable 64-bit floating point)
* integer (mutable 32-bit integer)
* coll (an immutable collection of reals)
* TODO: dict (key-value immutable collection of reals)

## usage
```bash $ ./mrdo ```

will start a repl. Use `:h` in the repl to get a list of meta-commands.

```bash $ ./mrdo <filename> [-o <output>] ```

will compile the program provided and run it, optionally outputting the
bytecode. If the provided program is already bytecode, it will be run
directly.

for other flags, see ```bash $ ./mrdo --help```

## submodules
**compiler** compiles from high-level to assembly

**asm** compiles from assembly to bytecode

**vm** runs the bytecode

**repl** understands both _assembly_ and _high level_ code

## language features

### variables
Variables can be defined and have values assigned using the `=` operator.
Assignment is a copy operation, ie:

```
foo = 42.0
bar = foo
```

will copy the value from `foo` to `bar`, resulting in two instances being
defined.

Also note that while the variable type is inferred, it is also immutable.
Once a variable is a type, it can't be reassigned to a new type.

### functions
TODO (not implemented in the new rust world yet)

Functions operate on collections in parallel and are either a:
* `map` (convert each element in the input collection to one element in the
output collection)
* `filter` (conditionally output each element in the input collection)
* `fold` (accumulate a collection to a single element)

For `map`, the input and output collections may be different types.

More function types may be added later.

### operations
The usual operations are available:

* arithmetical: +, -, /, *
* comparitive: `gt`, `gte`, `lt`, `lte`, `eq`, `ne`
* TODO: logical: `and`, `or`, `not`, `xor`
Logical operations treat 0.0 as false and all other values as true.

#### arithmetical operations
if left and right are `real` or `integer`, arithmetical operations work as 
expected.

if both are `coll`, they must be the same size and the operations are applied
pairwise.

if one is `real` or `integer` and one is `coll`, the `real` or `integer` is 
applied to every element in the `coll`.

#### comparitive operations
for `coll` types, comparisons follow the rustlang model. specifically, if any
element of a collection compares true for the operation, then the operation
as a whole will return true (or 1, actually).

#### collection-specific operations
Collections themselves have the following operations defined:
* TODO: flatten: takes multiple collections and combines them into a single
collection.
* TODO: sort: takes a collection and returns a seq of the elements in an order
governed by a given comparison operation.
* TODO: first: returns the first 'n' elements of a collection in the same
* collection
type.

### io
Collections can be read or written using the functions
* TODO: read
* write
which used stdin and stdout

old
--
There's an original version of this project that is more feature rich and
uses LLVM to compile to a binary, but that restricted what could be done at
runtime so it's been moved to `old`.
