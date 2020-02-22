#!/bin/bash
set -ex
javac src/Test.java
cargo build
clojure -J-Djava.library.path="target/debug" -J-Xcheck:jni src/test.clj
