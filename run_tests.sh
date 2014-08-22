#!/bin/bash

for t in test/*.do; do
	echo "Running $t"
	./mrdo --optimize=true --dump_module=false $t
	
	if [ $? -ne 0 ]; then
		break
	fi
done
