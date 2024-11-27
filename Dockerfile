FROM rust:1.82-bullseye

RUN apt-get update 


RUN apt-get install -y pkg-config  libssl-dev xgboost

RUN apt-get install libclang-dev -y

WORKDIR /app

COPY ./Cargo.toml /app

COPY ./Cargo.lock /app

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo run

ENV LIBCLANG_PATH=/usr/lib/llvm-11/lib

COPY . /app

ENTRYPOINT [ "cargo", "run" ]



