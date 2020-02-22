#!/bin/bash
set -ex
javac src/Test.java
cargo build
LD_LIBRARY_PATH=target/debug clojure -J-Xcheck:jni src/test.clj
