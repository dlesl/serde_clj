# serde_clj

Convert Rust data structures to/from (relatively) idiomatic Clojure data
in memory using JNI.

See [test/src/lib.rs](test/src/lib.rs) for a usage example.

## Example

```rust
#[derive(Serialize)]
struct {
    number: i32,
    names: Vec<String>
}
```
becomes
```clojure
{:number 3
 :names ["foo" "bar"]}
```

## Notes/TODO

* Unsigned integers serialize to the 'next biggest' type (except u64,
  which becomes i64), since Java doesn't really support unsigned.
  Maybe there's a better solution.
* Accordingly, if you want to serialize a `Vec<u8>` representing
  binary data, you should annotate or wrap these fields with
  [serde_bytes](https://crates.io/crates/serde_bytes), or you will end
  up with a vector of `java.lang.Short`, which probably isn't very
  efficient.
* Check that struct field names are keywords when deserializing.
* More extensive tests.
