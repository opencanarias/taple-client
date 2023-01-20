#!/bin/bash

## CONSTANTS
RED='\033[0;31m'
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m' 

set -e
# We request the node that wants to create the event
echo -e "${BLUE}Requesting port.${NC}"
echo -n "Enter the number ID of the node from which you want to perform the event: "
read number
port=$((10000+$number))
echo

# We ask for the subject_id of the subject to which you want to perform the event
echo -e "${BLUE}Requesting subject.${NC}"
echo -n -e "Enter the ID of the subject to which you want to perform an event ${RED}(example: Jjvs-Kk5FHRVwfktXEiH7y12CYZmV3sSBEyxwzECVA9Y)${NC}: "
read subject_id
echo

# Requesting temperature
echo -e "${BLUE}Requesting product temperature.${NC}"
echo -n -e "Enter the temperature at which the product is stored ${RED}(example: 30)${NC}: "
read temperature
echo

# Requesting location
echo -e "${BLUE}Requesting product location.${NC}"
echo -n -e "Enter the location where the product is located ${RED}(example: Spain)${NC}: "
read location
echo

# Requesting weight
echo -e "${BLUE}Requesting product weight.${NC}"
echo -n -e "Enter product weight ${RED}(example: 33)${NC}: "
read weight
echo

# Requesting the origin
echo -e "${BLUE}Requesting product origin.${NC}"
echo -n -e "Enter the product origin ${RED}(example: Spain)${NC}: "
read origin
echo

response=$(curl -s --location --request POST "http://localhost:${port}/api/requests" \
            --header 'Content-Type: application/json' \
            --data-raw "{
                \"request\": {
                    \"State\": {
                        \"subject_id\": \"${subject_id}\",
                        \"payload\": {
                            \"Json\": {
                                \"temperature\": ${temperature},
                                \"location\": \"${location}\",
                                \"batch\": {
                                    \"weight\": ${weight},
                                    \"origin\": \"${origin}\"
                                }
                            }
                        }
                    }
                }
            }")
set +e

echo -e "${GREEN}Response:${NC}"
echo  ${response}