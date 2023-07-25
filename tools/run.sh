#!/bin/bash

TOOL=""
ARGS=()

for var in "$@"
do
    if [ -z "${TOOL}" ] ; then
        TOOL=("$var")
    else
        ARGS+=("$var")
    fi
done

$TOOL "${ARGS[@]}"
