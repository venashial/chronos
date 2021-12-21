FROM rust:1.57.0-slim-buster

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/chronos"]
