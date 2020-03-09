//! # serde_clj
//! 
//! Convert Rust data structures to/from (relatively) idiomatic
//! Clojure data in memory using JNI.
//! 
//! See
//! [test/src/lib.rs](https://github.com/dlesl/serde_clj/blob/master/test/src/lib.rs)
//! for a usage example.
//! 
//! ## Example
//! 
//! ```rust
//! #[derive(Serialize)]
//! struct MyStruct {
//!     number: i32,
//!     names: Vec<String>
//! }
//! ```
//! becomes
//! ```clojure
//! {:number 3
//!  :names ["foo" "bar"]}


mod convert;
mod de;
mod error;
mod ser;

pub use convert::{Decoder, Encoder};
pub use de::{from_object, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_object, Serializer};
