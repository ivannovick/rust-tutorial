# Rust tutorial dev image.
#
# Extends the official Rust image with vim so you can edit lesson code
# directly inside the container.
#
# Build from the repo root:
#   docker build -t rust-tutorial:latest .
FROM rust:latest

RUN apt-get update \
    && apt-get install -y --no-install-recommends vim \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /work
