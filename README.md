do
===
do is a language in which variables may be either a:

* coll (an immutable collection of things)
* seq (an ordered immutable collection of things)
* dict (key-value immutable collection of things)

Each thing is a double, though more types may be added later.

functions
---------

Functions operate on these collections in parallel and are either a:

* map (convert each element in the input collection to one element in the output
    collection)
* filter (conditionally output each element in the input collection)

For map, the input and output collections may be different types.

More function types may be added later.

operations
----------
The usual operations are available within a function for elements of a
collection:

* arithmetical: +, -, /, *
* comparitive: gt, lt, eq
* logical: and, or, not

Collections themselves have the following operations defined:

* flatten: takes multiple collections and combines them into a single
collection.
* sort: takes a collection and returns a seq of the elements in an order
governed by a given comparison operation.
* first: returns the first 'n' elements of a collection in the same collection
type.


io
--
Collections can be read or written to files on disk using the functions

* read
* write

both of which take a name.
