FROM lukemathwalker/cargo-chef:latest-rust-1.89.0 as chef 
workdir /app
run apt update && apt install lld clang -y 

from chef as planner 

COPY . .

run cargo chef prepare --recipe-path recipe.json


from chef as builder 

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

ENV SQLX_OFFLINE true

Run cargo build --release --bin zero2prod

FROM debian:bookworm-slim as runtime

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \ 
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./target/release/zero2prod"]
