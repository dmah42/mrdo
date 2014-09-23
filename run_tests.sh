#!/bin/bash

for t in test/*.do; do
	echo "Running $t"

  OUTPUT=`./mrdo --optimize=true --dump_module=false $t 2> /dev/null`

	if [ $? -ne 0 ]; then
    echo "non-zero error code"
		break
  fi

  if [ "$OUTPUT" != "" ]; then
    echo "read '$OUTPUT'"

    GOLD_FILE=${t%.*}.out
    DIFF=""
    if [ -e $GOLD_FILE ]; then
      echo "checking '$OUTPUT' against '`cat $GOLD_FILE`' (from $GOLD_FILE)"
      DIFF=`diff <(echo "$OUTPUT") $GOLD_FILE`
      echo '((' $DIFF '))'
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
