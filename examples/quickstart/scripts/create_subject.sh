#!/bin/bash

## CONSTANTS
RED='\033[0;31m'
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m' 

set -e
# We request the node in which you want to create the subject
echo -e "${BLUE}Requesting port.${NC}"
echo -n "Enter the number ID of the node on which you want to create the subject: "
read number
port=$((10000+$number))
echo



# We request the governance_id
echo -e "${BLUE}Requesting governance.${NC}"
echo -n -e "Enter the ID of the governance in which you want to create the subject ${RED}(example: J1ZZ57u4PpvTl3apJ0BQrRFrQ1ftMC4XXg-kd9CkZC3E)${NC}: "
read governance_id
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

# Creating a subject at node one
response=$(curl -s --location --request POST "http://localhost:${port}/api/requests" \
            --header 'Content-Type: application/json' \
            --data-raw "{
                \"request\": {
                    \"Create\": {
                        \"governance_id\":  \"${governance_id}\",
                        \"namespace\": \"\",
                        \"schema_id\": \"Test\",
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

echo -e "${BLUE}Response:${NC} ${response}"
#subject_id=$(echo ${response} | grep -i -s -w -o -P "(?<=subject_id.{3})[^,\"]+")
#echo -e "${GREEN}Subject ID created:${NC} ${subject_id}"