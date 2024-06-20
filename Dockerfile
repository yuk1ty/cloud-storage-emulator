FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim as runner
WORKDIR /app

COPY --from=builder ./app/target/release/cloud-storage-emulator ./target/release/cloud-storage-emulator

ENV PORT 8000
EXPOSE $PORT

ENTRYPOINT ["./target/release/cloud-storage-emulator"]
