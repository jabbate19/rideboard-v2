FROM rust:1.79-bookworm AS base
RUN apt update && apt install -y libpq-dev

FROM base AS chef
RUN cargo install cargo-chef 
WORKDIR app

# FRONTEND BUILDER
FROM node:21 AS frontend

WORKDIR /app

COPY frontend/package.json .
COPY frontend/package-lock.json .

RUN npm install

COPY frontend .

RUN npm run build

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# BINARY BUILDER
FROM chef AS builder

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

COPY Cargo* .

COPY src src

COPY --from=frontend /app/dist/ /app/frontend/dist/

RUN cargo build --release

# RUNTIME
FROM base

WORKDIR /app

COPY --from=builder /app/target/release/rideboard-rewrite .

CMD ["./rideboard-rewrite"]