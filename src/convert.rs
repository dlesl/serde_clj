use crate::Result;

use jni::{
    objects::{JClass, JMethodID, JObject, JStaticMethodID, JValue},
    signature::{JavaType, Primitive},
    JNIEnv,
};

pub struct Encoder<'a> {
    pub env: JNIEnv<'a>,
    class_boolean: JClass<'a>,
    class_byte: JClass<'a>,
    class_integer: JClass<'a>,
    class_short: JClass<'a>,
    class_long: JClass<'a>,
    class_float: JClass<'a>,
    class_double: JClass<'a>,
    class_character: JClass<'a>,
    class_persistentvector: JClass<'a>,
    class_persistenthashmap: JClass<'a>,

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

    class_keyword: JClass<'a>,
    intern_keyword: JStaticMethodID<'a>,
    // cached_keywords: RefCell<HashMap<String, JObject<'a>>>,
    create_persistentvector: JStaticMethodID<'a>,
    create_persistenthashmap: JStaticMethodID<'a>,
}

impl<'a> Encoder<'a> {
    pub fn new(env: JNIEnv<'a>) -> Result<Self> {
        let class_boolean = env.find_class("java/lang/Boolean")?;
        let class_byte = env.find_class("java/lang/Byte")?;
        let class_integer = env.find_class("java/lang/Integer")?;
        let class_long = env.find_class("java/lang/Long")?;
        let class_short = env.find_class("java/lang/Short")?;
        let class_float = env.find_class("java/lang/Float")?;
        let class_double = env.find_class("java/lang/Double")?;
        let class_character = env.find_class("java/lang/Character")?;

        let class_arraylist = env.find_class("java/util/ArrayList")?;

        let class_keyword = env.find_class("clojure/lang/Keyword")?;

        let class_persistentvector = env.find_class("clojure/lang/PersistentVector")?;
        let class_persistenthashmap = env.find_class("clojure/lang/PersistentHashMap")?;

        Ok(Self {
            valueof_boolean: env.get_static_method_id(
                class_boolean,
                "valueOf",
                "(Z)Ljava/lang/Boolean;",
            )?,
            valueof_byte: env.get_static_method_id(class_byte, "valueOf", "(B)Ljava/lang/Byte;")?,
            valueof_integer: env.get_static_method_id(
                class_integer,
                "valueOf",
                "(I)Ljava/lang/Integer;",
            )?,
            valueof_long: env.get_static_method_id(class_long, "valueOf", "(J)Ljava/lang/Long;")?,
            valueof_short: env.get_static_method_id(
                class_short,
                "valueOf",
                "(S)Ljava/lang/Short;",
            )?,
            valueof_float: env.get_static_method_id(
                class_float,
                "valueOf",
                "(F)Ljava/lang/Float;",
            )?,
            valueof_double: env.get_static_method_id(
                class_double,
                "valueOf",
                "(D)Ljava/lang/Double;",
            )?,
            valueof_character: env.get_static_method_id(
                class_character,
                "valueOf",
                "(C)Ljava/lang/Character;",
            )?,

            new_arraylist: env.get_method_id(class_arraylist, "<init>", "()V")?,
            add_arraylist: env.get_method_id(class_arraylist, "add", "(Ljava/lang/Object;)Z")?,
            toarray_arraylist: env.get_method_id(
                class_arraylist,
                "toArray",
                "()[Ljava/lang/Object;",
            )?,

            intern_keyword: env.get_static_method_id(
                class_keyword,
                "intern",
                "(Ljava/lang/String;)Lclojure/lang/Keyword;",
            )?,

            create_persistentvector: env.get_static_method_id(
                class_persistentvector,
                "create",
                "(Ljava/lang/Iterable;)Lclojure/lang/PersistentVector;",
            )?,

            create_persistenthashmap: env.get_static_method_id(
                class_persistenthashmap,
                "create",
                "([Ljava/lang/Object;)Lclojure/lang/PersistentHashMap;",
            )?,

            class_boolean,
            class_byte,
            class_integer,
            class_long,
            class_short,
            class_float,
            class_double,
            class_character,

            class_arraylist,
            class_keyword,
            class_persistentvector,
            class_persistenthashmap,

            // cached_keywords: Default::default(),
            env,
        })
    }

    // Caching keywords here reduces JNI calls but also means we end
    // up holding too many local refs, so we would need some sort of
    // LRU cache. Maybe not worth it, need to benchmark.

    pub fn get_keyword(&self, name: &str) -> Result<JObject<'a>> {
        let s = self.env.auto_local(self.env.new_string(name)?.into());
        let k = self
            .env
            .call_static_method_unchecked(
                self.class_keyword,
                self.intern_keyword,
                JavaType::Object(String::new()),
                &[s.as_obj().into()],
            )?
            .l()?;
        Ok(k)
    }

    pub fn to_boxed(&self, val: JValue<'a>) -> Result<JObject<'a>> {
        let res = match val {
            JValue::Object(_) => val,
            JValue::Bool(_) => self.env.call_static_method_unchecked(
                self.class_boolean,
                self.valueof_boolean,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Byte(_) => self.env.call_static_method_unchecked(
                self.class_byte,
                self.valueof_byte,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Int(_) => self.env.call_static_method_unchecked(
                self.class_integer,
                self.valueof_integer,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Short(_) => self.env.call_static_method_unchecked(
                self.class_short,
                self.valueof_short,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Long(_) => self.env.call_static_method_unchecked(
                self.class_long,
                self.valueof_long,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Float(_) => self.env.call_static_method_unchecked(
                self.class_float,
                self.valueof_float,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Double(_) => self.env.call_static_method_unchecked(
                self.class_double,
                self.valueof_double,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Char(_) => self.env.call_static_method_unchecked(
                self.class_character,
                self.valueof_character,
                JavaType::Object(String::new()),
                &[val],
            )?,
            JValue::Void => JObject::null().into(),
        };
        Ok(res.l()?)
    }
}

/// This is a 'local ref safe' wrapper around ArrayList. It deletes
/// the reference to itself when converted to a PersistentVector or
/// PersistentHashMap, and consumes/deletes references to objects that
/// are added.
pub struct ArrayList<'a> {
    enc: &'a Encoder<'a>,
    obj: JObject<'a>,
}

impl<'a> ArrayList<'a> {
    pub fn new(enc: &'a Encoder<'a>) -> Result<Self> {
        Ok(Self {
            obj: enc
                .env
                .new_object_unchecked(enc.class_arraylist, enc.new_arraylist, &[])?,
            enc,
        })
    }

    /// This method will invalidate the local ref `val`!
    pub fn add(&self, val: JObject<'a>) -> Result<()> {
        let val = self.enc.env.auto_local(val);
        self.enc.env.call_method_unchecked(
            self.obj,
            self.enc.add_arraylist,
            JavaType::Primitive(Primitive::Boolean),
            &[val.as_obj().into()],
        )?;
        Ok(())
    }

    pub fn to_vector(self) -> Result<JObject<'a>> {
        let obj = self.enc.env.auto_local(self.obj);
        Ok(self
            .enc
            .env
            .call_static_method_unchecked(
                self.enc.class_persistentvector,
                self.enc.create_persistentvector,
                JavaType::Object(String::new()),
                &[obj.as_obj().into()],
            )?
            .l()?)
    }

    pub fn to_hashmap(self) -> Result<JObject<'a>> {
        let obj = self.enc.env.auto_local(self.obj);
        let arr = self.enc.env.auto_local(
            self.enc
                .env
                .call_method_unchecked(
                    obj.as_obj(),
                    self.enc.toarray_arraylist,
                    JavaType::Array(Box::new(JavaType::Object(String::new()))), // why is this necessary?
                    &[],
                )?
                .l()?,
        );
        Ok(self
            .enc
            .env
            .call_static_method_unchecked(
                self.enc.class_persistenthashmap,
                self.enc.create_persistenthashmap,
                JavaType::Object(String::new()),
                &[arr.as_obj().into()],
            )?
            .l()?)
    }
}
