<div align="center">
<p>⚠️ <b>TAPLE is in early development and <a href="https://www.taple.es/community/DISCLAIMER/">should not be used in production</a></b> ⚠️</p>
<br/>
<p><img src="https://raw.githubusercontent.com/opencanarias/public-resources/master/images/taple-logo-readme.png"></p>
</div>

## The main branch is the pre-release, development version of `TAPLE`. Please see the [0.1](https://github.com/opencanarias/taple-tools/tree/release-0.1) branch for the latest versions released.

# TAPLE Tools

TAPLE (pronounced T+🍎 ['tapəl]) stands for Tracking (Autonomous) of Provenance and Lifecycle Events. TAPLE is a permissioned DLT solution for traceability of assets and processes. It is:

- **Scalable**: Scaling to a sufficient level for traceability use cases. 
- **Light**: Designed to support resource constrained devices.
- **Flexible**: Have a flexible and adaptable cryptographic scheme mechanism for a multitude of scenarios.
- **Energy-efficient**: Rust powered, TAPLE is sustainable and efficient from the point of view of energy consumption.

TAPLE Tools is a set of tools that make it easier for the operator to manage a TAPLE network.

[![AGPL licensed][agpl-badge]][agpl-url]

[agpl-badge]: https://img.shields.io/badge/license-AGPL-blue.svg
[agpl-url]: https://github.com/opencanarias/taple-core/blob/master/LICENSE

[Technology](https://www.taple.es) | [Develop](https://www.taple.es/docs/develop) | [Core](https://github.com/opencanarias/taple-core) | [Client](https://github.com/opencanarias/taple-client) | [Tools](https://github.com/opencanarias/taple-tools)

## Usage
You can choose how to use the tools by either compiling the code and running them natively or through the Docker image.

Visit the [TAPLE Tools guide](https://www.taple.es/docs/develop/taple-tools) to learn how to use the tools.

### Build From Source
```bash
$ git clone https://github.com/opencanarias/taple-tools.git
$ cd taple-tools
$ cargo install --path taple-keygen
$ cargo install --path taple-sign
$ taple-keygen -h
$ taple-sign -h
```
## Docker images
Prebuilt docker images are available at [Docker Hub](https://hub.docker.com/r/opencanarias/taple-tools). The project includes [bash scripts](./scripts/) that allow for running utilities stored in the Docker image as if they were native applications. You will have to assign execution permissions to the scripts and add, optionally, add them to the path. 

```bash
$ git clone https://github.com/opencanarias/taple-tools.git
$ cd taple-tools
$ chmod +x ./scripts/taple-keygen
$ chmod +x ./scripts/taple-sign
$ ./scripts/taple-keygen -h
$ ./scripts/taple-sign -h
```

## License
This project is licensed under the [AGPL license](https://github.com/opencanarias/taple-core/blob/master/LICENSE).
