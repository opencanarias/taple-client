FROM rust:1.65 AS builder
WORKDIR /app
RUN apt update
RUN apt install -y libprotobuf-dev protobuf-compiler
RUN apt install cmake -y
COPY . tools
WORKDIR /app/tools
RUN sed -i '/"client"/d' Cargo.toml
RUN sed -i '/"settings"/d' Cargo.toml
RUN cargo install --path tools/keygen
RUN cargo install --path tools/sign
RUN cargo install --path tools/patch

FROM debian:buster-slim
WORKDIR /home
COPY --from=builder /usr/local/cargo/bin/taple-keygen /usr/local/bin/taple-keygen
COPY --from=builder /usr/local/cargo/bin/taple-sign /usr/local/bin/taple-sign
COPY --from=builder /usr/local/cargo/bin/taple-patch /usr/local/bin/taple-patch
COPY tools/run.sh ./run.sh
RUN chmod a+x run.sh
ENTRYPOINT ["./run.sh"]