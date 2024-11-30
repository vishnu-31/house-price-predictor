FROM rust:1.82-bullseye

RUN apt-get update 

RUN apt-get install -y pkg-config  libssl-dev xgboost

RUN apt-get install libclang-dev -y

WORKDIR /app

ENV LIBCLANG_PATH=/usr/lib/llvm-11/lib

COPY ./Cargo.toml /app

COPY ./Cargo.lock /app

RUN mkdir -p src/bin && echo "fn main() {}" > src/bin/train.rs

RUN echo "fn main() {}" > src/bin/api.rs

RUN cargo run --bin api


CMD [ "echo", "'rust-env'" ]

