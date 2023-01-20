#!/bin/bash

## CONSTANTS
RED='\033[0;31m'
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m' 

set -e
# We request the node to be checked
echo -e "${BLUE}Requesting port.${NC}"
echo -n -e "Enter the number ID of the node you want to check subjects on: "
read number
port=$((10000+$number))
echo

# We obtain the subjects of the bootstrap node
response=$(curl -s --location --request GET "http://localhost:${port}/api/subjects")
set +e

echo -e "${GREEN}Response:${NC}"
echo ${response}
