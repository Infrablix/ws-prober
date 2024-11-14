# Step 1: Build the application
FROM rust:1.82.0-alpine3.20 AS builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    gcc \
    libc-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

# Set OpenSSL static linking
ENV OPENSSL_STATIC=1
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include

# Copy only the Cargo.toml and Cargo.lock first to leverage Docker caching
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies and cache them
RUN cargo fetch

# Now copy the rest of the project
COPY . .

# Build the actual application
RUN cargo build --release

# Step 2: Run the application
FROM alpine:3.20.3 AS ws-prober-target

# Install run libraries
RUN apk add --no-cache \
    ca-certificates

# Copy the built executable from the builder stage
COPY --from=builder /usr/src/app/target/release/ws-prober /usr/local/bin/ws-prober

# Expose the port that your Actix Web server will run on
EXPOSE 9555

# Run the binary
CMD ["/usr/local/bin/ws-prober"]
