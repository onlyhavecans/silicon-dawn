FROM rust:1.57 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
EXPOSE 3200/tcp

COPY --from=builder /usr/local/cargo/bin/silicon-dawn /usr/local/bin/silicon-dawn
COPY templates /templates
COPY cards /cards
COPY config.toml /

CMD ["/usr/local/bin/silicon-dawn"]
