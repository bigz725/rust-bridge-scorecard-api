FROM rust:1.74.0 as builder

RUN mkdir /rust_bridge_scorecard_api
WORKDIR /rust_bridge_scorecard_api
COPY ./Cargo.toml ./Cargo.toml
RUN cargo new backend
COPY ./backend/Cargo.toml ./backend/Cargo.toml
RUN cargo build --release

RUN rm backend/src/*.rs
ADD ./backend ./backend
RUN rm ./target/release/deps/backend*
RUN cargo build --release

#rust_bridge_scorecard_api_backend
FROM debian:bookworm-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata gcc-aarch64-linux-gnu \
    && apt-get autoremove -y \
    && rm -rf /var/lib/apt/lists/*

ENV PORT 4040
EXPOSE ${PORT}

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder rust_bridge_scorecard_api/target/release/backend ${APP}/backend

# If we add static assets to the backend, we can uncomment the following line:
### CAUTION #############################################################
# Make sure to edit this line; the dockerfile has changed since this was commented out
# and has not been tested.  In particular, the paths are likely wrong.
########################################################################
#COPY --from=builder rust_bridge_scorecard_api_backend/static ${APP}/static

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./backend"]