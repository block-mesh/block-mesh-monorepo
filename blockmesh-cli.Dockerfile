# Stage 1: Build
FROM rust:latest as builder

WORKDIR /usr/src/app
ENV RUSTC_WRAPPER=""

COPY . .

RUN cargo build -p blockmesh-cli --release

# Stage 2
FROM ubuntu:latest

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/blockmesh-cli /usr/src/app/blockmesh-cli

RUN echo '#!/bin/bash\n\
exec /usr/src/app/blockmesh-cli --email "$EMAIL" --password "$PASSWORD"' > /usr/local/bin/entrypoint.sh && \
    chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
