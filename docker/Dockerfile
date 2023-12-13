ARG APP_NAME=kladez-api
ARG RUST_VERSION=1.73.0

# Build stage
FROM rust:${RUST_VERSION}-alpine3.18 AS builder

ARG APP_NAME

RUN apk add --no-cache musl-dev \
    && USER=root cargo new --bin ${APP_NAME}
WORKDIR /${APP_NAME}

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN echo "fn main() {}" > src/main.rs \
    && cargo build --release

# Build
COPY .sqlx ./.sqlx
COPY src ./src
RUN cargo build --release

# Final stage
FROM gcr.io/distroless/cc

ARG APP_NAME

ENV BIND_HOST=0.0.0.0
ENV BIND_PORT=80

USER nonroot:nonroot

COPY --from=builder /${APP_NAME}/target/release/${APP_NAME} /usr/local/bin/app

EXPOSE ${BIND_PORT}

HEALTHCHECK CMD wget -q --method=HEAD 127.0.0.1/health-check || exit 1

ENTRYPOINT ["/usr/local/bin/app"]
