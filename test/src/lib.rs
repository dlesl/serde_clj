use jni::objects::JClass;
use jni::sys::{jint, jobject};
use jni::JNIEnv;
use serde::Serialize;
use serde_clj::{to_object, Encoder};
use std::collections::HashMap;
use std::iter::repeat;

#[derive(Serialize, Clone)]
enum Vars<'a> {
    One(usize),
    Two(String),
    Three(&'a HashMap<i32, String>),
    Four { a: i32, b: bool, s: String },
}

#[derive(Serialize, Clone)]
struct Test<'a> {
    value: Vec<i64>,
    another_field: Option<String>,
    a_string: String,
    enumerate: Vec<Vars<'a>>,
}

#[no_mangle]
pub extern "system" fn Java_Test_test(env: JNIEnv, _: JClass, n: jint) -> jobject {
    let enc = Encoder::new(env).unwrap();
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(7, "test".into());
    let test = Test {
        value: vec![1, 2, 3],
        another_field: None,
        a_string: "test".into(),
        enumerate: vec![
            Vars::One(1),
            Vars::Two("three".into()),
            Vars::Three(&map),
            Vars::Four {
                a: 1,
                b: true,
                s: "ok?".into(),
            },
        ],
    };
    let vec = repeat(test).take(n as usize).collect::<Vec<_>>();
    let output = to_object(&enc, &vec).expect("serialisation failed!");
    output.into_inner()
}
