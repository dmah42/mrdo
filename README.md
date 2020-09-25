![Rust](https://github.com/dominichamon/mrdo/workflows/Rust/badge.svg)
# mrdo
mrdo is a language in which variables may be either a:
* real (mutable 64-bit floating point) coll (an immutable collection of reals)
* TODO: dict (key-value immutable collection of reals)

## usage
```bash $ ./mrdo ```

will start a repl (that currently only understands assembly).

```bash $ ./mrdo <filename> [-o <output>] ```

will compile the program provided and run it, optionally outputting the
bytecode. If the provided program is already bytecode, it will be run
directly.
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
TODO

Functions operate on collections in parallel and are either a:
* `map` (convert each element in the input collection to one element in the
output collection)
* `filter` (conditionally output each element in the input collection) `fold`
* (accumulate a collection to a single element)
For `map`, the input and output collections may be different types.

More function types may be added later.

### operations
The usual operations are available within a function for elements of a
collection (reals):
* arithmetical: +, -, /, * comparitive: `gt`, `ge`, `lt`, `le`, `eq`, `ne`
* TODO: logical: `and`, `or`, `not`, `xor`
Logical operations treat 0.0 as false and all other values as true.

#### arithmetical operations
if left and right are `real`, arithmetical operations work as expected.

if both are `coll`, they must be the same size and the operations are applied
pairwise.

if one is `real` and one is `coll`, the real is applied to every element in
the coll.

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
TODO: Collections can be read or written using the functions
* read write
which used stdin and stdout

old
--
There's an original version of this project that is more feature rich and
uses LLVM to compile to a binary, but that restricted what could be done at
runtime so it's been moved to `old`.