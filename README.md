<div align="center">
<p><b>This is main branch of work and is not guaranteed to be stable. Released versions are in the "release*" branches</b></p>
<br/>
<p><img src="https://raw.githubusercontent.com/opencanarias/public-resources/master/images/taple-logo-readme.png"></p>
</div>

# TAPLE Client
TAPLE (pronounced T+🍎 ['tapəl]) stands for Tracking (Autonomous) of Provenance and Lifecycle Events. TAPLE is a permissioned DLT solution for traceability of assets and processes. It is:

- **Scalable**: Scaling to a sufficient level for traceability use cases. 
- **Light**: Designed to support resource constrained devices.
- **Flexible**: Have a flexible and adaptable cryptographic scheme mechanism for a multitude of scenarios.
- **Energy-efficient**: Rust powered, TAPLE is sustainable and efficient from the point of view of energy consumption.

TAPLE Client is the reference application for connecting to the TAPLE DLT network. It is written in Rust and internally uses [TAPLE Core](https://github.com/opencanarias/taple-core) to implement the TAPLE protocol.

[![AGPL licensed][agpl-badge]][agpl-url]

[agpl-badge]: https://img.shields.io/badge/license-AGPL-blue.svg
[agpl-url]: https://github.com/opencanarias/taple-core/blob/master/LICENSE

[Discover](https://www.taple.es) | [Learn](https://www.taple.es/learn) | [Build](https://www.taple.es/build) | [Code](https://github.com/search?q=topic%3Ataple+org%3Aopencanarias++fork%3Afalse+archived%3Afalse++is%3Apublic&type=repositories)

## Build From Source
Rust versión 1.66 or higher is required.

```bash
$ git clone https://github.com/opencanarias/taple-client.git
$ cd taple-client
$ sudo apt install -y libprotobuf-dev protobuf-compiler cmake
$ rustup target add wasm32-unknown-unknown
$ cargo install --path client
$ taple-client --version
```

## Usage
Example of minimum configuration to start a node. An example identity is used and the REST API is activated. 
```sh
taple-client \
  --http \
  -k 7a747ddf55cf9b2ceb3b41a7c7ce9f88f835c120644e3c7522d97520668c8520
```

Refer to official TAPLE-Client [documentation](https://www.taple.es/docs/learn/taple-client) and [tutorials](https://www.taple.es/docs/build/taple-client) to learn how to set up and run the application.

## Docker images
Prebuilt docker images are available at [Docker Hub](https://hub.docker.com/r/opencanarias/taple-client).

If you want to build the image yourself, then you should do it in the following way:
```sh
docker build -f ./Dockerfile.client -t taple-client .
```

## Taple Tools
TAPLE Tools are a group of utilities designed to facilitate the use of TAPLE Client, especially during testing and prototyping. Look at this [README](./tools/README.md) for more information. 

## License
This project is licensed under the [AGPL license](./LICENSE).
