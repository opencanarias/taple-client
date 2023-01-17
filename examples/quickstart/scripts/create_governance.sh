#!/bin/bash

## CONSTANTS
RED='\033[0;31m'
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m' 

# Preguntamos por los numeros de los nodos que van a ser miembros de la gobernanza
# Este número servirá para identificar el nodo generado en el script anterior.
selected_nodes_controllerID=()
echo -e "${BLUE}Requesting controllerID.${NC}"
while :
do
   
    echo -n "Enter the controllerID of the node that will be inserted (empty to exit): "
    read controllerID
    if [ -z "${controllerID}" ]; then
        echo "<STOP>"
        echo
        break
    fi

    selected_nodes_controllerID+=(\"${controllerID}\")

done
controllers_ID_string=$(IFS=',' ; echo "${selected_nodes_controllerID[*]}")
echo

members=()
# Ahora procedemos a construir el JSON que define a los 3 miembros:
count=1
for controllerID in "${selected_nodes_controllerID[@]}"
do
    #echo ${selected_nodes_controllerID[*]}
    members+=("{
        \"id\": \"Company-$count\",
        \"tags\": {},
        \"description\": \"Headquarters in $count\",
        \"key\": $controllerID
    }")
    count=$(($count+1))
done
members_string=$(IFS=',' ; echo "${members[*]}")
#echo $members_string

echo -e "${BLUE}Requesting HTTP PORT.${NC}"
echo -n "Enter the number ID of the NODE where request for governance creation will be sent: "
read number
http_port=$((10000+$number))
echo


data_raw="{
        \"request\": {
            \"Create\": {
                \"governance_id\": \"\",
                \"namespace\": \"\",
                \"schema_id\": \"governance\",
                \"payload\": {
                    \"Json\": {
                        \"members\": [
                            ${members_string}
                        ],
                        \"schemas\": [
                            {
                                \"id\": \"Test\",
                                \"tags\": {},
                                \"content\": {
                                    \"type\": \"object\",
                                    \"additionalProperties\": false,
                                    \"required\": [
                                        \"temperature\",
                                        \"location\",
                                        \"batch\"
                                    ],
                                    \"properties\": {
                                        \"temperature\": {\"type\": \"integer\"},
                                        \"location\": {\"type\": \"string\" },
                                        \"batch\": {
                                            \"type\": \"object\",
                                            \"additionalProperties\": false,
                                            \"required\": [ \"weight\", \"origin\" ],
                                            \"properties\": {
                                                \"weight\": {\"type\": \"number\", \"minimum\": 0},
                                                \"origin\": {\"type\": \"string\"}
                                            }
                                        }
                                    }
                                }
                            }
                        ],
                        \"policies\": [
                            {
                                \"id\": \"governance\",
                                \"validation\": {
                                    \"quorum\": 0.5,
                                    \"validators\": [
                                        ${controllers_ID_string}
                                    ]
                                },
                                \"approval\": {
                                    \"quorum\": 0.5,
                                    \"approvers\": [
                                        ${controllers_ID_string}
                                    ]
                                },
                                \"invokation\": {
                                    \"owner\": { 
                                        \"allowance\": true,
                                        \"approvalRequired\": false
                                    },
                                    \"set\": {
                                        \"allowance\": false,
                                        \"approvalRequired\": false,
                                        \"invokers\": []
                                    },
                                    \"all\": {
                                        \"allowance\": false,
                                        \"approvalRequired\": false
                                    },
                                    \"external\": {
                                        \"allowance\": false,
                                        \"approvalRequired\": false
                                    }
                                }
                            },
                            {
                                \"id\": \"Test\",
                                \"validation\": {
                                    \"quorum\": 0.5,
                                    \"validators\": [
                                        ${controllers_ID_string}
                                    ]
                                },
                                \"approval\": {
                                    \"quorum\": 0.5,
                                    \"approvers\": [
                                        ${controllers_ID_string}
                                    ]
                                },
                                \"invokation\": {
                                    \"owner\": { 
                                        \"allowance\": true,
                                        \"approvalRequired\": false
                                    },
                                    \"set\": {
                                        \"allowance\": false,
                                        \"approvalRequired\": false,
                                        \"invokers\": []
                                    },
                                    \"all\": {
                                        \"allowance\": false,
                                        \"approvalRequired\": false
                                    },
                                    \"external\": {
                                        \"allowance\": false,
                                        \"approvalRequired\": false
                                    }
                                }
                            }
                        ]
                    }
                }
            }
        }
    }"
#echo $data_raw

resp=$(curl -s --location --request POST "http://localhost:$http_port/api/requests" \
    --header 'x-api-key: 1234' \
    --header 'Content-Type: application/json' \
    --data-raw "${data_raw}")

echo $resp
