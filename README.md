mrdo
===
[![Build Status](https://travis-ci.org/dominichamon/mrdo.svg?branch=master)](https://travis-ci.org/dominichamon/mrdo)

mrdo is a language in which variables may be either a:

* real (mutable 64-bit floating point)
* coll (an immutable collection of reals)
* seq (an ordered immutable collection of reals)
* dict (key-value immutable collection of reals)

Other types than real may be added later.

functions
---------

Functions operate on these collections in parallel and are either a:

* map (convert each element in the input collection to one element in the output
    collection)
* filter (conditionally output each element in the input collection)
* fold (accumulate a collection to a single element)

For map, the input and output collections may be different types.

More function types may be added later.

operations
----------
The usual operations are available within a function for elements of a
collection:

* arithmetical: +, -, /, *
* comparitive: gt, ge, lt, le, eq, ne
* logical: and, or, not, xor

Logical operations treat 0.0 as false and all other values as true.

Collections themselves have the following operations defined:

* TODO: flatten: takes multiple collections and combines them into a single
collection.
* TODO: sort: takes a collection and returns a seq of the elements in an order
governed by a given comparison operation.
* TODO: first: returns the first 'n' elements of a collection in the same collection
type.

io
--
Collections can be read or written using the functions

* read
* write

which used stdin and stdout

vm
--
This package compiles the code down to assembly defined in the
[mrdovm](https://github.com/dominichamon/mrdovm) project. It can then be
run using that vm.

old
--
There's an original version of this project that used LLVM to compile to a
binary, but that restricted what could be done at runtime so it's been moved
to `old`.
