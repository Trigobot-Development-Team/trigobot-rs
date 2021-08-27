# Create builder image
from ekidd/rust-musl-builder:stable as builder

# Build dependencies apart from our code
RUN cargo new --bin trigobot
WORKDIR ./trigobot
COPY --chown=rust:rust ./Cargo.lock ./Cargo.lock
COPY --chown=rust:rust ./Cargo.toml ./Cargo.toml
RUN cargo build --release && rm -r src/*.rs

# Build our code
ADD --chown=rust:rust ./src ./src
ADD --chown=rust:rust ./proto ./proto
COPY --chown=rust:rust ./build.rs ./build.rs
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/trigobot* && cargo build --release


# Create image to run our code
FROM alpine:latest

ENV RUN_USER=trigobot

RUN addgroup -S $RUN_USER && adduser -S -g $RUN_USER $RUN_USER
COPY --from=builder --chown=$RUN_USER:$RUN_USER /home/rust/src/trigobot/target/x86_64-unknown-linux-musl/release/trigobot /home/trigobot/trigobot

RUN chown $RUN_USER:$RUN_USER /home/trigobot/trigobot
#COPY --chown=$RUN_USER:$RUN_USER /home/trigobot/.env /.env

USER $RUN_USER
WORKDIR /home/trigobot

CMD ["./trigobot"]
