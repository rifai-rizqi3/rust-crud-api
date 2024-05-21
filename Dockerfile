# Build Stage
FROM rust:1.69-buster as builder

WORKDIR /app

# Accept the build argument
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
COPY . .
RUN cargo build --release

# Production Stage
FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY --from=builder /app/target/release/rust-crud-api .
CMD [ "./rust-crud-api" ]