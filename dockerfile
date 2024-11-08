FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY . .

RUN cargo fetch
RUN cargo install cargo-watch cargo-script

COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

CMD ["cargo-watch", "-w", "leet", "-x", "script src/leet/two-sum.rs"]
