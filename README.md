## This is TAPLE's main branch of work and is not guaranteed to be stable. Released versions are in the "release*" branches

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

[Discover](https://www.taple.es) | [Learn](https://www.taple.es/learn) | [Build](https://www.taple.es/build) | 
[Code](https://github.com/search?q=topic%3Ataple+org%3Aopencanarias++fork%3Afalse+archived%3Afalse++is%3Apublic&type=repositories)

## Build From Source
Rust versión 1.65 or higher is required.

```bash
$ git clone https://github.com/opencanarias/taple-client.git
$ cd taple-client
$ apt install -y libprotobuf-dev protobuf-compiler
$ apt install cmake -y
$ cargo install --path client
$ taple-client --version
```

## Docker images
Prebuilt docker images are available at [Docker Hub](https://hub.docker.com/r/opencanarias/taple-client).

If you want to build the images yourself, then you should do it in the following way:
```sh
DOCKER_BUILDKIT=1 docker build -f /path/to/Dockerfile .
```

You can build both the image of the client and the image of the TAPLE tools. Both Dockerfile can be found in the build directory. The command should be executed from the root directory.

## Usage
Refer to official TAPLE-Client [documentation](https://www.taple.es/docs/learn/taple-client) and [tutorials](https://www.taple.es/docs/build/taple-client) to learn how to set up and run the application.

## Taple Tools
This repository also contains the set of available Taple tools. You can consult the "tools" directory for more information about them.

## License
This project is licensed under the [AGPL license](./LICENSE).
