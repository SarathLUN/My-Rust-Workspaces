FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:latest AS runner
RUN apt-get update \
     && apt-get -y install libpq-dev gcc
EXPOSE 8080
COPY --from=builder /app/target/release/eventsapi /app/target/release/eventsapi
COPY migrations /app/target/release/migrations
CMD ["/app/target/release/eventsapi"]