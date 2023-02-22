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
$ cargo install --path client
$ taple-client --version
```

## Docker images
Prebuilt docker images are available at [Docker Hub](https://hub.docker.com/r/opencanarias/taple-client).

## Usage
Refer to our [quick start guide and developer documentation](https://www.taple.es/docs/develop) to learn how to set up and run the application.

## License
This project is licensed under the [AGPL license](https://github.com/opencanarias/taple-core/blob/master/LICENSE).