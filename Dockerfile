# Create builder image
FROM alpine:latest AS builder

# Install rust
RUN apk add rustup musl-dev gcc
RUN rustup-init -y

# docker build won't source ~/.profile for some reason
ENV PATH=/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

# we always build from scratch, so disable incremental builds
ENV CARGO_INCREMENTAL=0


# Build dependencies apart from our code
RUN cargo new --bin trigobot
WORKDIR ./trigobot
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release && rm -r src/*.rs

# Build our code
ADD ./src ./src
RUN rm ./target/release/deps/trigobot* && cargo build --release


# Create image to run our code
FROM alpine:latest

COPY --from=builder /trigobot/target/release/trigobot /home/trigobot/trigobot

WORKDIR /home/trigobot/run

CMD ["../trigobot"]
