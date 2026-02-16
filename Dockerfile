FROM rust:1.93-bookworm AS builder

WORKDIR /workdir

# Compile the project
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=crates,target=crates \
    --mount=type=bind,source=js,target=js \
    --mount=type=bind,source=migrations,target=migrations \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/workdir/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo install --locked --path .


FROM debian:bookworm-slim AS runtime

ENV RUST_LOG=info

WORKDIR /opt/fmbq-website
COPY --from=builder /usr/local/cargo/bin/fmbq-website /usr/local/bin/fmbq-website
COPY wwwroot wwwroot

CMD ["fmbq-website"]
