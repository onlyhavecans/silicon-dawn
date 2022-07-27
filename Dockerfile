FROM lukemathwalker/cargo-chef:latest-rust-1.62.1 as chef
WORKDIR /usr/src/myapp

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json


FROM chef as builder
COPY --from=planner /usr/src/myapp/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim as production
EXPOSE 3200/tcp

ENV USER=appuser
ENV UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --no-create-home \
    --uid "$UID" \
    "$USER"

COPY cards /cards
COPY config.toml /
COPY --from=builder /usr/local/cargo/bin/silicon-dawn /usr/local/bin/silicon-dawn

USER appuser

CMD ["/usr/local/bin/silicon-dawn"]

#HEALTHCHECK --interval=1m --timeout=1s \
#    CMD curl -f http://localhost:3200/healthcheck || exit 1
