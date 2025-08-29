FROM lukemathwalker/cargo-chef:latest-rust-1.89@sha256:abbe80c8000f4e1b6969b4d84d5ec7ad86616be7e6322ba0e3b451c2eee6f280 AS chef
WORKDIR /usr/src/myapp

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /usr/src/myapp/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian12:latest@sha256:d4e8c4c18626ce7c09104d3b39d9e5541ced61de7fb398a455cb09ae5a7b3598 AS production
EXPOSE 3200/tcp

COPY cards /cards
COPY config.toml /
COPY --from=builder /usr/local/cargo/bin/silicon-dawn /usr/local/bin/silicon-dawn

USER nonroot

CMD ["/usr/local/bin/silicon-dawn"]

#HEALTHCHECK --interval=1m --timeout=1s \
#    CMD curl -f http://localhost:3200/healthcheck || exit 1
