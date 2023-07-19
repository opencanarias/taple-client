#!/bin/bash

if [ $# == 0 ]; then
  echo -e "${RED}No arguments passed${NC}"
  exit 1
else
  TOOL=$1
  $TOOL ${@:2}
fi
