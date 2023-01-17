#!/bin/bash

## CONSTANTS
RED='\033[0;31m'
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m' 

docker container stop $(docker container ls -q --filter name=node-*)

echo -e "${GREEN}Containers stopped and disposed of correctly${NC}"
