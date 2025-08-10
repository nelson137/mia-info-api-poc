######################################################################
# Planner

FROM instrumentisto/rust:nightly-bookworm-slim-2025-08-08 AS planner

RUN cargo install cargo-chef

WORKDIR /app

COPY . .

RUN cargo chef prepare --bin mia-info-poc --recipe-path recipe.json

######################################################################
# Builder

FROM instrumentisto/rust:nightly-bookworm-slim-2025-08-08 AS builder

RUN apt update &&\
    apt install -y --no-install-recommends curl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN cargo install cargo-chef

# Copy the build plan from the previous Docker stage
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this layer is cached as long as `recipe.json`
# doesn't change.
RUN cargo chef cook --release --bin mia-info-poc --features=loki --recipe-path recipe.json

# Build the whole project
COPY . .

RUN cargo build --release --bin mia-info-poc --features=loki

######################################################################
# Runtime

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/mia-info-poc .
COPY --from=builder /app/config/ ./config/

EXPOSE 8080

CMD ["/app/mia-info-poc"]
