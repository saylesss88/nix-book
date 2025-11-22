#!/bin/sh
cat > /tmp/pre.json
/var/home/jr/nix-book/mdbook-rss/target/release/mdbook-rss "$@" < /tmp/pre.json
