# TAPLE Tools

TAPLE Tools are a group of utilities designed to facilitate the use of TAPLE Client, especially during testing and prototyping. TAPLE Tools are included within TAPLE Client repository. 

## Build From Source

Rust versión 1.66 or higher is required.

```bash
$ git clone https://github.com/opencanarias/taple-client.git
$ cd taple-client
$ sudo apt install -y libprotobuf-dev protobuf-compiler cmake
$ cargo install --path tools/taple-keygen
$ cargo install --path tools/taple-patch
$ cargo install --path tools/taple-sign
$ taple-keygen -h
$ taple-sign -h
$ taple-patch -h
```

## Usage
Visit the [TAPLE Tools guide](https://www.taple.es/docs/learn/client-tools) to learn how to use the tools.

## Docker images
Prebuilt docker images are available at [Docker Hub](https://hub.docker.com/r/opencanarias/taple-tools).

If you want to build the image yourself, then you should do it in the following way:
```sh
docker build -f ./Dockerfile.tools -t taple-tools .
```

## License
This project is licensed under the [AGPL license](https://github.com/opencanarias/taple-core/blob/master/LICENSE).
