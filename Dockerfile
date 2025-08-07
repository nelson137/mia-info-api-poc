######################################################################
# Planner

FROM instrumentisto/rust:nightly-bookworm-slim-2025-02-01 AS planner

RUN cargo install cargo-chef

WORKDIR /app

COPY . .

RUN cargo chef prepare --bin mia-info-poc --recipe-path recipe.json

######################################################################
# Builder

FROM instrumentisto/rust:nightly-bookworm-slim-2025-02-01 AS builder

WORKDIR /app

RUN cargo install cargo-chef

# Copy the build plan from the previous Docker stage
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this layer is cached as long as `recipe.json`
# doesn't change.
RUN cargo chef cook --release --bin mia-info-poc --recipe-path recipe.json

# Build the whole project
COPY . .

RUN cargo build --release --bin mia-info-poc

######################################################################
# Runtime

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/mia-info-poc .
COPY --from=builder /app/config/ ./config/

EXPOSE 8080

CMD ["/app/mia-info-poc"]
