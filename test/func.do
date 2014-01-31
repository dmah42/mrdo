input = [1, 2, 3]
sum = fold { return cum + in; }
mean = do(sum, input} / do(length, input)
do(write, mean)

