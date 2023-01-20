#!/bin/bash

## CONSTANTS
RED='\033[0;31m'
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m' 

# We start by downloading/updating the docker image of TAPLE
docker pull opencanarias/taple-client:0.1
echo -e "${GREEN}Image downloaded successfully.${NC}"

# We ask for a number, this number will be used to define the HTTP port and the P2P port to be assigned to the node
echo -n "Enter a number to identify the node that will be deployed: "
read number
echo

http_port=$((10000+$number))
p2p_port=$((11000+$number))
database_path="/tmp/data"$number

# We check if the port is already in use and we leave if it is...
nc -z 127.0.0.1 $http_port
if [ $? -eq 0 ]; then
    echo "PORT already being used. Specify another number"
    exit 1;
fi

# We ask for the cryptographic material to start the TAPLE node
echo -e "${BLUE}Requesting cryptographic material.${NC}"
echo -n "Enter secret key: "
read secret_key

# We ask if the node is going to connect to a bootstrap (or not)
array=()
echo -e "${BLUE}Requesting know nodes.${NC}"
while :
do
    echo -n "Enter address of known node (leave empty to stop requesting):"
    read know_node
    if [ -z "${know_node}" ]; then
        echo -n "<STOP>"
        echo
        break
    fi
    array+=($know_node)
done
echo
know_nodes=$(IFS=';' ; echo "${array[*]}") # Concatenate the elements of the array with ';' to pass it later to the command

# Run TAPLE
docker_image_name="opencanarias/taple-client:0.1"
docker_container_name="node-$number"

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
$SCRIPT_DIR/__internal_launch_node.sh $docker_container_name $database_path $http_port $p2p_port $secret_key $docker_image_name $know_nodes