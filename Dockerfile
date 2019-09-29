FROM rust:1.38-alpine3.10 as build
WORKDIR /app
RUN cargo install --color never cargo-junit
COPY . .

RUN cargo build --color never --release
RUN cargo junit --name test-results.xml

FROM alpine:3.10
WORKDIR /app

RUN apk --no-cache add ca-certificates && \
    addgroup -S app && adduser -S app -G app

EXPOSE 8080
EXPOSE 9102

ENV DEBUG_LEVEL "DEBUG"

COPY --from=build /app/target/*/release/app /app/test-results.xml /app/

USER app
CMD ["./app"]
