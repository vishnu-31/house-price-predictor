FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

# Install additional OS packages
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    pkg-config \
    libssl-dev \
    libclang-dev

ENV LIBCLANG_PATH=/usr/lib/llvm-11/lib
