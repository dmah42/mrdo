#!/bin/bash

for t in test/*.do; do
	echo "Running $t"

  INPUT_FILE=${t%.*}.in
  OUTPUT=""
  if [ -e $INPUT_FILE ]; then
    # echo "using '$INPUT_FILE'"
    OUTPUT=`./mrdo --optimize=true --dump_module=false $t < $INPUT_FILE 2> /dev/null`
  else
    OUTPUT=`./mrdo --optimize=true --dump_module=false $t 2> /dev/null`
  fi

	if [ $? -ne 0 ]; then
    echo "non-zero error code"
		break
  fi

  if [ "$OUTPUT" != "" ]; then
    GOLD_FILE=${t%.*}.out
    DIFF=""
    if [ -e $GOLD_FILE ]; then
      # echo "checking '$OUTPUT' against '`cat $GOLD_FILE`' (from $GOLD_FILE)"
      DIFF=`diff <(echo "$OUTPUT") $GOLD_FILE`
      if [ "$DIFF" != "" ]; then
        echo "expected `cat ${t%.*}.out`, got $OUTPUT"
        break
      fi
    else
      echo "missing expected gold file $GOLD_FILE"
      break
    fi
  fi
done
