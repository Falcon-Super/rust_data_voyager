# First stage: build environment
FROM rust:1.73-alpine as builder

# Create appuser
RUN adduser -D -g '' appuser

WORKDIR /usr/src/rustdatavoyage
COPY . .

# Build the application
RUN cargo build --release
# Assuming your binary is named rustdatavoyage, if not, replace with the actual name
RUN cargo install --path .

# Second stage: setup the runtime environment
FROM alpine:3.18

# Install dependencies (if any)
# If your application has no dependencies, the following line can be omitted
RUN apk add --no-cache libgcc

# Import from builder
COPY --from=builder /usr/local/cargo/bin/rustdatavoyage /usr/local/bin/rustdatavoyage
COPY --from=builder /etc/passwd /etc/passwd

# Use an unprivileged user
USER appuser

# Command to run
CMD ["rustdatavoyage"]
