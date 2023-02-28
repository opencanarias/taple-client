FROM rust:1.65 as builder
WORKDIR /app
RUN apt update
RUN apt install -y libprotobuf-dev protobuf-compiler
RUN apt install cmake -y
COPY . taple_client
WORKDIR /app/taple_client
RUN cargo install --path client

FROM debian:buster-slim
WORKDIR /home
COPY --from=builder /usr/local/cargo/bin/taple_client /usr/local/bin/taple_client
CMD ["taple_client"]
