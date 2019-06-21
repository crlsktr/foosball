#!/bin/sh
git pull
pkill foosball_server
./backup.sh
cargo run --bin foosball_server &