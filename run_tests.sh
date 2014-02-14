#!/bin/bash

for t in test/*.do; do
	echo "Running $t"
	./mrdo $t
	
	if [ $? -ne 0 ]; then
		break
	fi
done
