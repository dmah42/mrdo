do(write, [1, 2, 3])

input = [3, 2, 1]
do(write, input)

# define func up front
sum = func(cum, in) { return cum + in }
mean = do(fold, sum, input) / do(length, input)

# define func inline
mean2 = do(fold, func(cum, in) { return cum + in }, input)

# map example with extra args
sqdiff = do(map, func(in, mean) {
	diff = in - mean
	return diff * diff
}, input, mean)
stdev = do(fold, sum, sqdiff) / do(length, sqdiff)

do(write, [mean, mean2, stdev])

