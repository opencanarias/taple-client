#!/bin/bash
#set -e #Salir inmediatamente en caso de error

## CONSTANTS
RED='\033[0;31m'
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m' 

# Empezamos descargando/actualizando la imagen docker de TAPLE
docker pull opencanarias/taple-client:0.1
echo -e "${GREEN}Image downloaded successfully.${NC}"

# Preguntamos un numero, este número servirá para definir el puerto HTTP y el puerto P2P que se asignará al nodo
echo -e "${BLUE}Requesting number.${NC}"
echo -n "Enter a number to identify the node that will be deployed: "
read number
echo

http_port=$((10000+$number))
p2p_port=$((11000+$number))
database_path="/tmp/data"$number

# Miramos si el puerto ya esta en uso y salimos si lo esta...
nc -z 127.0.0.1 $http_port
if [ $? -eq 0 ]; then
    echo "PORT already being used. Specify another number"
    exit 1;
fi

# Pedimos el material criptografico para iniciar el nodo TAPLE
echo -e "${BLUE}Requesting cryptographic material.${NC}"
echo -n "Enter secret key: "
read secret_key

#Preguntamos si el nodo se va a conectar a un bootstrap (o no)
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
know_nodes=$(IFS=';' ; echo "${array[*]}") #Concatenamos los elementos del array con ';' para pasarselo mas adelante al comando
#echo $know_nodes

# Ejecutamos TAPLE
docker_image_name="opencanarias/taple-client:0.1"
docker_container_name="node-$number"

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
$SCRIPT_DIR/__internal_launch_node.sh $docker_container_name $database_path $http_port $p2p_port $secret_key $docker_image_name $know_nodes
# command="docker run --network=host --name=${docker_container_name} -e RUST_LOG=info -e TAPLE_DATABASE_PATH=${database_path} -e TAPLE_HTTPPORT=${http_port} -e TAPLE_NETWORK_P2PPORT=${p2p_port} -e TAPLE_NODE_SECRETKEY=${secret_key} ${docker_image_name}"
# if [ -n "${know_nodes}" ]; then
#     # Si se han definido know nodes hay que cambiar el comando que se va a lanzar...
#     command="docker run --network=host --name=${docker_container_name} -e RUST_LOG=info -e TAPLE_DATABASE_PATH=${database_path} -e TAPLE_HTTPPORT=${http_port} -e TAPLE_NETWORK_P2PPORT=${p2p_port} -e TAPLE_NODE_SECRETKEY=${secret_key} -e TAPLE_NETWORK_KNOWNNODES=${know_nodes} ${docker_image_name}"
# fi
# #echo $command

# (
#     $command;
#     sleep 1;
#     docker rm ${docker_container_name} -f;
# )&

# Mostramos por consola los valores útiles
# sleep 1
# p2p_address=$(docker logs ${docker_container_name} 2>&1 | grep -i -s -w -o -P "(?<=RED:\s).+")
# peerID=$(docker logs ${docker_container_name} 2>&1 | tail -n 1 | grep -i -s -w -o -P "(?<=p2p/)(\d|\w)+")
# controllerID=$(docker logs ${docker_container_name} 2>&1 | grep -i -s -w -o -P "(?<=controller ID:\s).+")
# echo -e "${GREEN}Copy and save those values:${NC}"
# echo -e "${BLUE}NumberId :${NC} \t${number}"
# echo -e "${BLUE}ContainerID:${NC} \t${docker_container_name}"
# echo -e "${BLUE}ControllerID :${NC} \t${controllerID}"
# echo -e "${BLUE}HTTP Port :${NC} \t${http_port}"
# echo -e "${BLUE}PeerId :${NC} \t${peerID}"
# echo -e "${BLUE}P2P Addresses :${NC} \t[\n${p2p_address}\n]"