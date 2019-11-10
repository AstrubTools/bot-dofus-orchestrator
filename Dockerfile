FROM rustlang/rust:nightly
WORKDIR /app

RUN apt update -y && apt install mongodb

COPY *.toml .
COPY Cargo.lock .
RUN cargo build --release

COPY src src
RUN cargo build --release

EXPOSE 8080
CMD cargo run --release