#!/usr/bin/env bash
set -ex
javac src/Test.java
cargo build --release
clojure -J-Djava.library.path="target/release" src/bench.clj
