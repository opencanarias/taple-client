FROM rust:1.65 as builder
WORKDIR /app
RUN apt update
RUN apt install -y libprotobuf-dev protobuf-compiler
RUN apt install cmake -y
COPY . taple-client
WORKDIR /app/taple-client
RUN cargo install --path client

FROM debian:buster-slim
WORKDIR /home
COPY --from=builder /usr/local/cargo/bin/taple-client /usr/local/bin/taple-client
CMD ["taple-client"]
