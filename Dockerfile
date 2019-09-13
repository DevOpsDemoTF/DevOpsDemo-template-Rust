FROM rust:1.37-alpine

RUN cargo install cargo-test-junit

WORKDIR /app
COPY . .

RUN cargo build --release
RUN cargo test-junit --name test-results.xml

FROM alpine:3.10
WORKDIR /app

RUN addgroup -S app && adduser -S app -G app

EXPOSE 8080
EXPOSE 9102

ENV DEBUG_LEVEL "DEBUG"

COPY --from=build /app/target/release/app /app/test-results.xml /app/

USER app
CMD ["./app"]
