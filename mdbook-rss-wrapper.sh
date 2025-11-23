#!/bin/sh
# Use /bin/sh instead of /bin/bash for a simpler shell environment
# Absolute path to your functional binary (using single quotes to protect it)
BINARY_PATH='/var/home/jr/nix-book/mdbook-rss/target/release/mdbook-rss'
# Execute the binary directly with its absolute path,
# ensuring all I/O is passed through correctly.
# We are no longer using sed or cat, as they introduce another layer of shell complexity.
# We trust the binary's output is correct based on your manual tests.
exec "$BINARY_PATH" "$@" <&0
