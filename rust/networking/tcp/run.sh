#!/bin/bash

cargo b --release

sudo setcap cap_net_admin=eip /home/duxa/RUST-learn/NetWorking/target/release/NetWorking

cleanup() {
  echo "Caught signal, terminating..."
  kill "$pid"
  wait "$pid"
}

trap cleanup SIGINT SIGTERM

/home/duxa/RUST-learn/NetWorking/target/release/NetWorking &
pid=$!

sudo ip addr add 192.168.0.2/24 dev test
sudo ip link set up dev test

wait "$pid"
