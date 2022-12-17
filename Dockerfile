# Build stage
FROM rust:latest as builder
WORKDIR /app
ADD . /app
# migration
COPY .env.prod .env
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres
RUN sqlx database create
RUN sqlx migrate run
# application
RUN cargo build --release

# Prod stage
FROM gcr.io/distroless/cc
# copy configuration
COPY default.json /
COPY production.json /
COPY log4rs.yml /
COPY --from=builder /app/target/release/dashboard-server /
ENV DEV_BOARD_ENV production
CMD ["./dashboard-server"]