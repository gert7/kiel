#!/bin/sh

sudo apt install libpq-dev postgresql libssl-dev docker.io
cargo install diesel_cli --no-default-features --features "postgres"

