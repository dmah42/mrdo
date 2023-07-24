(3.4 + 1.0) - 2.8
42.0 / 1.3
31.0 / 3.0 neq 42.0 - 2.0
foo = 41.0 + 1.0
foo = foo / 2.0
; let's write it out
do(write, foo)

bar = [42.0, 3, 1]
baz = bar + foo
do(write, bar)
do(write, baz)

func foobar(qux: real) {
    do(write, qux)
}
