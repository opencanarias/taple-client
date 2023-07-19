FROM rust:1.67-slim-buster as builder
WORKDIR /app
RUN apt update
RUN apt install -y build-essential
RUN apt install -y libprotobuf-dev protobuf-compiler
RUN apt install cmake -y
RUN rustup target add wasm32-unknown-unknown
COPY . taple-client
WORKDIR /app/taple-client
RUN sed -i '/"tools\/keygen"/d' Cargo.toml
RUN sed -i '/"tools\/patch"/d' Cargo.toml
RUN sed -i '/"tools\/sign"/d' Cargo.toml
RUN mkdir -p "contracts"
RUN mkdir -p "db"
ENV TAPLE_SC_BUILD_PATH "/app/taple-client/contracts"
ENV TAPLE_DBPATH "/app/taple-client/db"
RUN cargo install --path client
# RUN chmod +777 /app/taple-client/contracts
CMD ["taple-client"]
