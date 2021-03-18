use jni::objects::{JClass, JObject};
use jni::sys::{jint, jobject, jstring};
use jni::JNIEnv;
use json_benchmark::{canada::Canada,twitter::Twitter};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_clj::{from_object, to_object, Decoder, Encoder};
use std::collections::HashMap;
use std::iter::repeat;

#[derive(Deserialize, Serialize, Clone, Debug)]
enum Vars {
    Zero,
    One(usize),
    Two(String),
    Three(HashMap<i32, String>),
    Four { a: i32, b: bool, s: String },
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Test {
    value: Vec<i64>,
    another_field: Option<String>,
    a_string: String,
    // This gets encoded as a 1 char string, since java's char type
    // does not support all utf8 characters
    a_char: char,
    tuple: (i32, String),
    opt_tuple: (i32, Option<String>),
    enumerate: Vec<Vars>,
    #[serde(with = "serde_bytes")]
    bytes: Vec<u8>,
}

#[no_mangle]
pub extern "system" fn Java_Test_ser(env: JNIEnv, _: JClass, n: jint) -> jobject {
    let enc = Encoder::new(env).unwrap();
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(7, "test".into());
    let test = Test {
        value: vec![1, 2, 3],
        another_field: None,
        a_string: "test".into(),
        a_char: '𩸽',
        tuple: (4, "hey".into()),
        opt_tuple: (2, None),
        enumerate: vec![
            Vars::Zero,
            Vars::One(1),
            Vars::Two("three".into()),
            Vars::Three(map),
            Vars::Four {
                a: 1,
                b: true,
                s: "ok?".into(),
            },
        ],
        bytes: vec![0, 1, 2],
    };
    let vec = repeat(test).take(n as usize).collect::<Vec<_>>();
    let output = to_object(&enc, &vec).expect("serialisation failed!");
    output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_Test_de(env: JNIEnv, _: JClass, obj: JObject) {
    let dec = Decoder::new(env).unwrap();
    let out: Vec<Test> = from_object(&dec, obj).expect("deserialisation failed");
    println!("{:?}", out);
}

#[no_mangle]
pub extern "system" fn Java_Test_roundtrip(env: JNIEnv, _: JClass, obj: JObject) -> jobject {
    // making an encoder and a decoder uses a lot of local refs to
    // cache class & method ids
    env.ensure_local_capacity(64)
        .expect("failed increasing capacity");
    let dec = Decoder::new(env.clone()).unwrap();
    let out: Vec<Test> = from_object(&dec, obj).expect("deserialisation failed");
    let enc = Encoder::new(env).unwrap();
    let output = to_object(&enc, &out).expect("serialisation failed!");
    output.into_inner()
}

// Benchmarks

lazy_static! {
    static ref CANADA: Canada =
        serde_json::from_slice(include_bytes!("../data/canada.json")).unwrap();
    static ref TWITTER: Twitter =
        serde_json::from_slice(include_bytes!("../data/twitter.json")).unwrap();
}

/// call this once first to prepare the lazy static
#[no_mangle]
pub extern "system" fn Java_Test_canadaSer(env: JNIEnv, _: JClass) -> jobject {
    let enc = Encoder::new(env).unwrap();
    let output = to_object(&enc, &CANADA as &Canada).expect("serialisation failed!");
    output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_Test_canadaSerJson(env: JNIEnv, _: JClass) -> jstring {
    let output = serde_json::to_string(&CANADA as &Canada).expect("serialisation failed!");
    env.new_string(output).unwrap().into_inner()
}

#[no_mangle]
pub extern "system" fn Java_Test_canadaDe(env: JNIEnv, _: JClass, obj: JObject) {
    let dec = Decoder::new(env).unwrap();
    let _: Canada = from_object(&dec, obj).expect("deserialisation failed");
}

#[no_mangle]
pub extern "system" fn Java_Test_twitterSer(env: JNIEnv, _: JClass) -> jobject {
    let enc = Encoder::new(env).unwrap();
    let output = to_object(&enc, &TWITTER as &Twitter).expect("serialisation failed!");
    output.into_inner()}

#[no_mangle]
pub extern "system" fn Java_Test_twitterDe(env: JNIEnv, _: JClass, obj: JObject) {
    let dec = Decoder::new(env).unwrap();
    let _: Twitter = from_object(&dec, obj).expect("deserialisation failed");
}
