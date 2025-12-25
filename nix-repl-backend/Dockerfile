FROM rust:1.85-slim as builder

WORKDIR /build
COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

# Runtime image
FROM debian:stable-slim

RUN apt-get update && \
    apt-get install -y curl ca-certificates xz-utils tini && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m -s /bin/bash nixuser
RUN mkdir -m 0755 /nix && chown nixuser /nix

USER nixuser
ENV USER=nixuser
ENV NIX_INSTALLER_NO_MODIFY_PROFILE=1

# Install Nix
RUN curl -L https://nixos.org/nix/install -o /tmp/install-nix.sh \
    && sh /tmp/install-nix.sh --no-daemon \
    && rm /tmp/install-nix.sh

RUN mkdir -p /home/nixuser/.config/nix && \
    echo 'experimental-features = nix-command flakes' > /home/nixuser/.config/nix/nix.conf

ENV NIX_PATH=/home/nixuser/.nix-profile/etc/nix
ENV PATH=/home/nixuser/.nix-profile/bin:$PATH

WORKDIR /app

# Copy binary from builder
COPY --from=builder --chown=nixuser:nixuser /build/target/release/nix-repl-server /app/nix-repl-server

EXPOSE 8080

ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["/app/nix-repl-server"]
