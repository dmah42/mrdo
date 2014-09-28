#!/bin/bash

exec 6>&2         # save stderr to fd6
exec 2> /dev/null # redirect stderr to null

while getopts ":vh" opt; do
  case $opt in
    v)
      exec 2>&6 6>&-  # restore stderr and close fd6
      ;;
    h)
      echo "Usage: $0 [-v]"
      echo "  -v: show verbose output from mrdo"
      exit 0
      ;;
    \?)
      echo "unknown option: -$OPTARG"
      exec 2>&6 6>&-  # restore stderr and close fd6
      exit 1
      ;;
  esac
done

for t in test/*.do; do
	echo "Running $t"

  INPUT_FILE=${t%.*}.in
  if [ -e $INPUT_FILE ]; then
    # echo "using '$INPUT_FILE'"
    exec < $INPUT_FILE
  fi
  OUTPUT=`./mrdo --optimize=true --dump_module=false $t`

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
