FROM rust:1.74.0 as builder

RUN USER=root cargo new --bin rust_bridge_scorecard_api_backend
WORKDIR /rust_bridge_scorecard_api_backend
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
ADD . ./
RUN rm ./target/release/deps/rust_bridge_scorecard_api_backend*
RUN cargo build --release

#rust_bridge_scorecard_api_backend
FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata gcc-aarch64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

ENV PORT 4040
EXPOSE ${PORT}

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /rust_bridge_scorecard_api_backend/target/release/rust_bridge_scorecard_api_backend ${APP}/rust_bridge_scorecard_api_backend
# If we add static assets to the backend, we can uncomment the following line:
#COPY --from=builder /rust_bridge_scorecard_api_backend/static ${APP}/static

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./rust_bridge_scorecard_api_backend"]