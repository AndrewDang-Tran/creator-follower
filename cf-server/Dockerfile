FROM rust:1.62-bullseye

WORKDIR /usr/src/creator-follower
COPY . .

RUN cargo install --path .

CMD ["cf-server"]
