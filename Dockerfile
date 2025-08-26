FROM lukemathwalker/cargo-chef:latest-rust-1.75@sha256:49a8981fbfa9a024b4d1824a40b66bf84d23f087e70751a4de07cac0c53f3882 as chef
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

FROM gcr.io/distroless/cc-debian11@sha256:55a5e011b2c4246b4c51e01fcc2b452d151e03df052e357465f0392fcd59fddf as production
EXPOSE 3200/tcp

COPY cards /cards
COPY config.toml /
COPY --from=builder /usr/local/cargo/bin/silicon-dawn /usr/local/bin/silicon-dawn

USER nonroot

CMD ["/usr/local/bin/silicon-dawn"]

#HEALTHCHECK --interval=1m --timeout=1s \
#    CMD curl -f http://localhost:3200/healthcheck || exit 1
