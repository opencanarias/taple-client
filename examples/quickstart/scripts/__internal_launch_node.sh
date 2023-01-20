#!/bin/sh

docker_container_name=$1
database_path=$2
http_port=$3
p2p_port=$4
secret_key=$5
docker_image_name=$6
know_nodes=$7

command="docker run --rm --network=host --name=${docker_container_name} -e RUST_LOG=info -e TAPLE_DATABASE_PATH=${database_path} -e TAPLE_HTTPPORT=${http_port} -e TAPLE_NETWORK_P2PPORT=${p2p_port} -e TAPLE_NODE_SECRETKEY=${secret_key} ${docker_image_name}"
if [ -n "${know_nodes}" ]; then
    # If you have defined know nodes, you must change the command to be launched...
    command="docker run --rm --network=host --name=${docker_container_name} -e RUST_LOG=info -e TAPLE_DATABASE_PATH=${database_path} -e TAPLE_HTTPPORT=${http_port} -e TAPLE_NETWORK_P2PPORT=${p2p_port} -e TAPLE_NODE_SECRETKEY=${secret_key} -e TAPLE_NETWORK_KNOWNNODES=${know_nodes} ${docker_image_name}"
fi

(
    $command;
    sleep 1;
    docker rm ${docker_container_name} -f;
)