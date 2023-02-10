##### Builder
FROM rust:1.61.0-slim as builder

WORKDIR /usr/src

# Create blank project
RUN USER=root 