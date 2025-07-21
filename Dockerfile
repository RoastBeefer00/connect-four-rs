FROM rust:latest AS rust-builder

WORKDIR /app

# Install musl target and tools
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools

COPY . .
WORKDIR /app/connect-four-server

# Build with musl target (statically linked)
RUN cargo build --release --target x86_64-unknown-linux-musl

# ---

FROM alpine:latest AS final

WORKDIR /app

# Copy the musl binary instead
COPY --from=rust-builder /app/target/x86_64-unknown-linux-musl/release/connect_four_server ./connect_four_server
RUN mkdir dist
COPY ./connect-four-server/dist ./dist

RUN chmod +x ./connect_four_server

EXPOSE 3000
CMD ["./connect_four_server"]
