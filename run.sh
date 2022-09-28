#!/bin/sh

cargo build -q;

sudo ./target/debug/keylogger;
