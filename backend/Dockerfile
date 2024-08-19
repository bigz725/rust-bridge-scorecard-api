FROM lukemathwalker/cargo-chef:latest-rust-1.80.1 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json



FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
#RUN cargo chef cook --release --recipe-path recipe.json

# Copy all files from our working environment to the image
COPY . .

ENV SQLX_OFFLINE true
# Build the project

RUN cargo update axum tower tower-layer
RUN cargo build --release --bin backend

# next debian release name is trixie, so make the next line
# FROM debian:trixie-slim AS runtime 
# when it's officially released
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt update -y \
    && apt install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend backend
COPY configuration configuration

ENTRYPOINT ["./backend"]