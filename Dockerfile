FROM rust:1.66-slim-buster as builder
WORKDIR /app
RUN apt update
RUN apt install -y build-essential
RUN apt install -y libprotobuf-dev protobuf-compiler
RUN apt install cmake -y
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-gc
ENV TAPLE_CONTRACTSDKPATH "/app/taple-client/contracts"
COPY . taple-client
WORKDIR /app/taple-client
RUN cargo install --path client
RUN chmod +777 /app/taple-client/contracts
CMD ["taple-client"]

# FROM debian:buster-slim
# WORKDIR /home
# COPY --from=builder /usr/local/cargo/bin/taple-client /usr/local/bin/taple-client
# CMD ["taple-client"]
