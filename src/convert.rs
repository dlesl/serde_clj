use crate::Result;

use jni::{
    objects::{AutoLocal, JClass, JMethodID, JObject, JStaticMethodID, JValue},
    signature::{JavaType, Primitive},
    JNIEnv,
};

pub(crate) struct Common<'a> {
    pub(crate) env: JNIEnv<'a>,
    pub(crate) class_boolean: JClass<'a>,
    pub(crate) class_byte: JClass<'a>,
    pub(crate) class_integer: JClass<'a>,
    pub(crate) class_short: JClass<'a>,
    pub(crate) class_long: JClass<'a>,
    pub(crate) class_float: JClass<'a>,
    pub(crate) class_double: JClass<'a>,
    pub(crate) class_character: JClass<'a>,
    pub(crate) class_string: JClass<'a>,
    pub(crate) class_persistentvector: JClass<'a>,
    pub(crate) class_persistenthashmap: JClass<'a>,
    pub(crate) class_imapiterable: JClass<'a>,
    pub(crate) class_keyword: JClass<'a>,
}

impl<'a> Common<'a> {
    pub fn new(env: JNIEnv<'a>) -> Result<Self> {
        Ok(Self {
            class_boolean: env.find_class("java/lang/Boolean")?,
            class_byte: env.find_class("java/lang/Byte")?,
            class_integer: env.find_class("java/lang/Integer")?,
            class_long: env.find_class("java/lang/Long")?,
            class_short: env.find_class("java/lang/Short")?,
            class_float: env.find_class("java/lang/Float")?,
            class_double: env.find_class("java/lang/Double")?,
            class_character: env.find_class("java/lang/Character")?,
            class_string: env.find_class("java/lang/String")?,
            class_keyword: env.find_class("clojure/lang/Keyword")?,
            class_persistentvector: env.find_class("clojure/lang/PersistentVector")?,
            class_persistenthashmap: env.find_class("clojure/lang/PersistentHashMap")?,
            class_imapiterable: env.find_class("clojure/lang/IMapIterable")?,
            env,
        })
    }
}

pub struct Encoder<'a> {
    pub(crate) com: Common<'a>,
    valueof_boolean: JStaticMethodID<'a>,
    valueof_byte: JStaticMethodID<'a>,
    valueof_integer: JStaticMethodID<'a>,
    valueof_short: JStaticMethodID<'a>,
    valueof_long: JStaticMethodID<'a>,
    valueof_float: JStaticMethodID<'a>,
    valueof_double: JStaticMethodID<'a>,
    valueof_character: JStaticMethodID<'a>,

    class_arraylist: JClass<'a>,
    new_arraylist: JMethodID<'a>,
    add_arraylist: JMethodID<'a>,
    toarray_arraylist: JMethodID<'a>,

    intern_keyword: JStaticMethodID<'a>,

    create_persistentvector: JStaticMethodID<'a>,
    create_persistenthashmap: JStaticMethodID<'a>,
}

impl<'a> Encoder<'a> {
    pub fn new(env: JNIEnv<'a>) -> Result<Self> {
        let com = Common::new(env)?;

        let class_arraylist = com.env.find_class("java/util/ArrayList")?;
        Ok(Self {
            valueof_boolean: com.env.get_static_method_id(
                com.class_boolean,
                "valueOf",
                "(Z)Ljava/lang/Boolean;",
            )?,
            valueof_byte: com.env.get_static_method_id(
                com.class_byte,
                "valueOf",
                "(B)Ljava/lang/Byte;",
            )?,
            valueof_integer: com.env.get_static_method_id(
                com.class_integer,
                "valueOf",
                "(I)Ljava/lang/Integer;",
            )?,
            valueof_long: com.env.get_static_method_id(
                com.class_long,
                "valueOf",
                "(J)Ljava/lang/Long;",
            )?,
            valueof_short: com.env.get_static_method_id(
                com.class_short,
                "valueOf",
                "(S)Ljava/lang/Short;",
            )?,
            valueof_float: com.env.get_static_method_id(
                com.class_float,
                "valueOf",
                "(F)Ljava/lang/Float;",
            )?,
            valueof_double: com.env.get_static_method_id(
                com.class_double,
                "valueOf",
                "(D)Ljava/lang/Double;",
            )?,
            valueof_character: com.env.get_static_method_id(
                com.class_character,
                "valueOf",
                "(C)Ljava/lang/Character;",
            )?,

            new_arraylist: com.env.get_method_id(class_arraylist, "<init>", "()V")?,
            add_arraylist: com.env.get_method_id(
                class_arraylist,
                "add",
                "(Ljava/lang/Object;)Z",
            )?,
            toarray_arraylist: com.env.get_method_id(
                class_arraylist,
                "toArray",
                "()[Ljava/lang/Object;",
            )?,
            class_arraylist,
            intern_keyword: com.env.get_static_method_id(
                com.class_keyword,
                "intern",
                "(Ljava/lang/String;)Lclojure/lang/Keyword;",
            )?,

            create_persistentvector: com.env.get_static_method_id(
                com.class_persistentvector,
                "create",
                "(Ljava/lang/Iterable;)Lclojure/lang/PersistentVector;",
            )?,

            create_persistenthashmap: com.env.get_static_method_id(
                com.class_persistenthashmap,
                "create",
                "([Ljava/lang/Object;)Lclojure/lang/PersistentHashMap;",
            )?,

            com,
        })
    }

    pub(crate) fn get_keyword(&'a self, name: &str) -> Result<JObject<'a>> {
        let s = self
            .com
            .env
            .auto_local(self.com.env.new_string(name)?);
        let k = self
            .com
            .env
            .call_static_method_unchecked(
                self.com.class_keyword,
                self.intern_keyword,
                JavaType::Object(String::new()),
                &[s.as_obj().into()],
            )?
            .l()?;
        Ok(k)
    }

    #[inline]
    pub(crate) fn to_boxed(&self, val: JValue<'a>) -> Result<JObject<'a>> {
        let com = &self.com;
        let res = match val {
            JValue::Object(_) => val,
            JValue::Bool(_) => com.env.call_static_method_unchecked(
                com.class_boolean,
                self.valueof_boolean,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Byte(_) => com.env.call_static_method_unchecked(
                com.class_byte,
                self.valueof_byte,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Int(_) => com.env.call_static_method_unchecked(
                com.class_integer,
                self.valueof_integer,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Short(_) => com.env.call_static_method_unchecked(
                com.class_short,
                self.valueof_short,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Long(_) => com.env.call_static_method_unchecked(
                com.class_long,
                self.valueof_long,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Float(_) => com.env.call_static_method_unchecked(
                com.class_float,
                self.valueof_float,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Double(_) => com.env.call_static_method_unchecked(
                com.class_double,
                self.valueof_double,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Char(_) => com.env.call_static_method_unchecked(
                com.class_character,
                self.valueof_character,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Void => JObject::null().into(),
        };
        Ok(res.l()?)
    }
}

pub(crate) struct ArrayList<'a> {
    enc: &'a Encoder<'a>,
    obj: AutoLocal<'a, 'a>,
}

impl<'a> ArrayList<'a> {
    pub fn new(enc: &'a Encoder<'a>) -> Result<Self> {
        Ok(Self {
            obj: enc.com.env.auto_local(enc.com.env.new_object_unchecked(
                enc.class_arraylist,
                enc.new_arraylist,
                &[],
            )?),
            enc,
        })
    }

    /// This method will invalidate the local ref `val`!
    pub fn add(&self, val: JObject<'a>) -> Result<()> {
        let val = self.enc.com.env.auto_local(val);
        self.enc.com.env.call_method_unchecked(
            self.obj.as_obj(),
            self.enc.add_arraylist,
            JavaType::Primitive(Primitive::Boolean),
            &[val.as_obj().into()],
        )?;
        Ok(())
    }

    pub fn to_vector(self) -> Result<JObject<'a>> {
        Ok(self
            .enc
            .com
            .env
            .call_static_method_unchecked(
                self.enc.com.class_persistentvector,
                self.enc.create_persistentvector,
                JavaType::Object(String::new()),
                &[self.obj.as_obj().into()],
            )?
            .l()?)
    }

    pub fn to_hashmap(self) -> Result<JObject<'a>> {
        let arr = self.enc.com.env.auto_local(
            self.enc
                .com
                .env
                .call_method_unchecked(
                    self.obj.as_obj(),
                    self.enc.toarray_arraylist,
                    JavaType::Array(Box::new(JavaType::Object(String::new()))), // why is this necessary?
                    &[],
                )?
                .l()?,
        );
        Ok(self
            .enc
            .com
            .env
            .call_static_method_unchecked(
                self.enc.com.class_persistenthashmap,
                self.enc.create_persistenthashmap,
                JavaType::Object(String::new()),
                &[arr.as_obj().into()],
            )?
            .l()?)
    }
}

pub struct Decoder<'a> {
    pub(crate) com: Common<'a>,
    pub(crate) value_boolean: JMethodID<'a>,
    pub(crate) value_byte: JMethodID<'a>,
    pub(crate) value_integer: JMethodID<'a>,
    pub(crate) value_short: JMethodID<'a>,
    pub(crate) value_long: JMethodID<'a>,
    pub(crate) value_float: JMethodID<'a>,
    pub(crate) value_double: JMethodID<'a>,

    pub(crate) class_rt: JClass<'a>,
    pub(crate) first_seq: JStaticMethodID<'a>,
    pub(crate) next_seq: JStaticMethodID<'a>,

    pub(crate) keyiterator_imapiterable: JMethodID<'a>,
    pub(crate) valiterator_imapiterable: JMethodID<'a>,

    pub(crate) getname_keyword: JMethodID<'a>,

    pub(crate) hasnext_iter: JMethodID<'a>,
    pub(crate) next_iter: JMethodID<'a>,

    /// a byte array
    pub(crate) class_bytes: JClass<'a>,
}

macro_rules! decode {
    ($func:ident, $out:ident, $class:ident, $value_method:ident, $prim:ident, $code:ident) => {
        pub(crate) fn $func(&self, obj: JObject) -> Result<Option<$out>> {
            if self
                .com
                .env
                .is_instance_of(obj, self.com.$class)?
            {
                    Ok(Some(self
                        .decode_prim(obj, self.$value_method, Primitive::$prim)?
                        .$code()?))
            } else {
                Ok(None)
            }
        }
    }
}

impl<'a> Decoder<'a> {
    pub fn new(env: JNIEnv<'a>) -> Result<Self> {
        let com = Common::new(env.clone())?;
        let class_rt = env.find_class("clojure/lang/RT")?;
        let class_iter = env.find_class("java/util/Iterator")?;
        let hasnext_iter = env.get_method_id(class_iter, "hasNext", "()Z")?;
        let next_iter = com
                .env
                .get_method_id(class_iter, "next", "()Ljava/lang/Object;")?;

        Ok(Decoder {
            value_boolean: com
                .env
                .get_method_id(com.class_boolean, "booleanValue", "()Z")?,
            value_byte: env.get_method_id(com.class_byte, "byteValue", "()B")?,
            value_float: com
                .env
                .get_method_id(com.class_float, "floatValue", "()F")?,
            value_double: com
                .env
                .get_method_id(com.class_double, "doubleValue", "()D")?,
            value_short: com
                .env
                .get_method_id(com.class_short, "shortValue", "()S")?,
            value_integer: com
                .env
                .get_method_id(com.class_integer, "intValue", "()I")?,
            value_long: env.get_method_id(com.class_long, "longValue", "()J")?,
            // seq_rt: env.get_static_method_id(class_rt, "seq", "(Ljava/lang/Object;)Lclojure/lang/ISeq;")?,
            first_seq: env.get_static_method_id(
                class_rt,
                "first",
                "(Ljava/lang/Object;)Ljava/lang/Object;",
            )?,
            next_seq: env.get_static_method_id(
                class_rt,
                "next",
                "(Ljava/lang/Object;)Lclojure/lang/ISeq;",
            )?,

            keyiterator_imapiterable: env.get_method_id(
                com.class_imapiterable,
                "keyIterator",
                "()Ljava/util/Iterator;",
            )?,

            valiterator_imapiterable: env.get_method_id(
                com.class_imapiterable,
                "valIterator",
                "()Ljava/util/Iterator;",
            )?,

            getname_keyword: env.get_method_id(
                com.class_keyword,
                "getName",
                "()Ljava/lang/String;",
            )?,

            hasnext_iter,
            next_iter,

            class_bytes: env.find_class("[B")?,
            class_rt,
            // class_iseq,
            com,
        })
    }

    fn decode_prim(
        &self,
        obj: JObject<'a>,
        method: JMethodID<'a>,
        ret: Primitive,
    ) -> Result<JValue<'a>> {
        Ok(self
            .com
            .env
            .call_method_unchecked(obj, method, JavaType::Primitive(ret), &[])?)
    }

    decode!(decode_f32, f32, class_float, value_float, Float, f);
    decode!(decode_f64, f64, class_double, value_double, Double, d);
    decode!(decode_i64, i64, class_long, value_long, Long, j);
    decode!(decode_i32, i32, class_integer, value_integer, Int, i);
    decode!(decode_i16, i16, class_short, value_short, Short, s);
    decode!(decode_bool, bool, class_boolean, value_boolean, Boolean, z);
    decode!(decode_i8, i8, class_byte, value_byte, Byte, b);

    pub(crate) fn decode_string(&self, obj: JObject) -> Result<Option<String>> {
        if self.com.env.is_instance_of(obj, self.com.class_string)? {
            Ok(Some(self.com.env.get_string(obj.into())?.into()))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn decode_bytes(&self, obj: JObject) -> Result<Option<Vec<u8>>> {
        if self.com.env.is_instance_of(obj, self.class_bytes)? {
            Ok(Some(self.com.env.convert_byte_array(obj.into_inner())?))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn decode_keyword(&self, obj: JObject) -> Result<Option<String>> {
        if self.com.env.is_instance_of(obj, self.com.class_keyword)? {
            let name = self.com.env.auto_local(
                self.com
                    .env
                    .call_method_unchecked(
                        obj,
                        self.getname_keyword,
                        JavaType::Object(String::new()),
                        &[],
                    )?
                    .l()?,
            );
            let res = self.com.env.get_string(name.as_obj().into())?.into();
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn map_to_iters(&self, obj: AutoLocal<'a, 'a>) -> Result<Option<(AutoLocal, AutoLocal)>> {
        if self
            .com
            .env
            .is_instance_of(obj.as_obj(), self.com.class_imapiterable)?
        {
            let key_iter = self.com.env.auto_local(
                self.com
                    .env
                    .call_method_unchecked(
                        obj.as_obj(),
                        self.keyiterator_imapiterable,
                        JavaType::Object(String::new()),
                        &[obj.as_obj().into()],
                    )?
                    .l()?,
            );

            let val_iter = self.com.env.auto_local(
                self.com
                    .env
                    .call_method_unchecked(
                        obj.as_obj(),
                        self.valiterator_imapiterable,
                        JavaType::Object(String::new()),
                        &[obj.as_obj().into()],
                    )?
                    .l()?,
            );

            Ok(Some((key_iter, val_iter)))
        } else {
            Ok(None)
        }
    }
}
