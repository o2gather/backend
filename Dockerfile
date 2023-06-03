FROM rust:1-bullseye AS builder
WORKDIR /app
COPY src ./src
COPY Cargo.toml .
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo build --release
RUN chmod +x ./target/release/backend

FROM ubuntu:22.04 AS runtime
WORKDIR /app
RUN adduser --disabled-password user && apt update && \
    apt install -y postgresql-client=14+238 curl=7.81.0-1ubuntu1.10 && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder --chown=user:user /app/target/release/backend .
ENV STAGE=production
EXPOSE 8080
USER user
CMD [ "/app/backend" ]