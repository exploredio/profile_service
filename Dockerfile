FROM rust:1.73 as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

# Create a new build directory for dependencies to speed up builds
RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release
RUN rm -f src/main.rs

COPY . .

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/profile_service /usr/local/bin/profile_service

EXPOSE 8080

CMD ["profile_service"]
