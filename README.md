
<div align="center">
<p>⚠️ <b>TAPLE is in early development and <a href="https://www.taple.es/docs/community/disclaimer">should not be used in production</a></b> ⚠️</p>
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

[Technology](https://www.taple.es) | [Develop](https://www.taple.es/docs/develop) | [Core](https://github.com/opencanarias/taple-core) | [Client](https://github.com/opencanarias/taple-client) | [Tools](https://github.com/opencanarias/taple-tools)

## Build From Source
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

## Usage
Refer to our [tutorials and developer documentation](https://taple.es/docs/develop/tutorial-from-0/introduction) to learn how to set up and run the application. However, this documentation will indicate the minimum commands to start a node both from the binary and with the Docker image.

Also visit the following [page](https://taple.es/docs/develop/taple-client-config) if you want to check the existing configurations for a TAPLE node.

### Binary usage
Once installed as described in section [Build From Source](#build-from-source):
```bash
# Generate an ED25519 secret key in hexadecimal format. TAPLE Keygen tool can be used with that purpose. Let's suppose 20a3e9463869c57a8d3e950d2ba7c1b51a10e97a446d9c3eba2e5da8e07a6f44
taple-client -k 20a3e9463869c57a8d3e950d2ba7c1b51a10e97a446d9c3eba2e5da8e07a6f44
```
### Docker usage
```bash
docker run --name taple1 \
  -e TAPLE_NODE_SECRETKEY=af9e38bbe732fe67071ee349f6a9bdc4ad0e5b9ef3518666bb273bd580d8d346 \
  -e RUST_LOG=info \
  -p 50000:50000 -p 3000:3000 opencanarias/taple-client
```

Port 50000 is the default port for protocol communications, while port 3000 is the port for the REST API.

## License
This project is licensed under the [AGPL license](https://github.com/opencanarias/taple-core/blob/master/LICENSE).