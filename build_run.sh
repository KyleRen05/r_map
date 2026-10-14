#!/bin/bash

cargo build
sudo setcap cap_net_raw=ep target/debug/main

cargo run main