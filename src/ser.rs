use jni::objects::JObject;
use serde::{self, ser, Serialize};

use crate::convert::{ArrayList, Encoder};
use crate::error::{Error, Result};

pub struct Serializer<'a> {
    enc: &'a Encoder<'a>,
}

impl<'a> Serializer<'a> {}

// based on https://github.com/serde-rs/json/blob/master/src/value/ser.rs

pub fn to_object<'a, T>(enc: &'a Encoder<'a>, value: &T) -> Result<JObject<'a>>
where
    T: Serialize,
{
    let serializer = Serializer { enc };
    value.serialize(serializer)
}

macro_rules! boxer {
    ($func:ident, $type:ty) => {
        boxer!($func, $type as $type);
    };
    ($func:ident, $type:ty as $as:ty) => {
        #[inline]
        fn $func(self, val: $type) -> Result<JObject<'a>> {
            self.enc.to_boxed((val as $as).into())
        }
    }
}

pub fn variant_map<'a>(
    enc: &'a Encoder<'a>,
    variant: &str,
    value: JObject<'a>,
) -> Result<JObject<'a>> {
    let res = ArrayList::new(enc)?;
    res.add(enc.get_keyword(variant)?)?;
    res.add(value)?;
    Ok(res.to_hashmap()?.into())
}

impl<'a> serde::Serializer for Serializer<'a> {
    type Ok = JObject<'a>;
    type Error = Error;

    type SerializeSeq = SerializeVec<'a>;
    type SerializeTuple = SerializeVec<'a>;
    type SerializeTupleStruct = SerializeVec<'a>;
    type SerializeTupleVariant = SerializeTupleVariant<'a>;
    type SerializeMap = SerializeVec<'a>;
    type SerializeStruct = SerializeVec<'a>;
    type SerializeStructVariant = SerializeStructVariant<'a>;

    boxer!(serialize_bool, bool);

    boxer!(serialize_i8, i8);
    boxer!(serialize_i16, i16);
    boxer!(serialize_i32, i32);
    boxer!(serialize_i64, i64);

    boxer!(serialize_u8, u8 as i16);
    boxer!(serialize_u16, u16 as i32);
    boxer!(serialize_u32, u32 as i64);
    boxer!(serialize_u64, u64 as i64); // TODO: BigInt here

    boxer!(serialize_f32, f32);
    boxer!(serialize_f64, f64);

    #[inline]
    fn serialize_char(self, value: char) -> Result<JObject<'a>> {
        // Rust's chars can be 32 bit so we need to encode as string
        let mut s = String::new();
        s.push(value);
        self.serialize_str(&s)
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<JObject<'a>> {
        Ok(self.enc.com.env.new_string(value)?.into())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<JObject<'a>> {
        Ok(self.enc.com.env.byte_array_from_slice(value)?.into())
    }

    #[inline]
    fn serialize_unit(self) -> Result<JObject<'a>> {
        Ok(JObject::null())
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<JObject<'a>> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<JObject<'a>> {
        // just a bare keyword
        self.enc.get_keyword(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<JObject<'a>>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<JObject<'a>>
    where
        T: Serialize,
    {
        variant_map(self.enc, variant, to_object(self.enc, &value)?)
    }

    #[inline]
    fn serialize_none(self) -> Result<JObject<'a>> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<JObject<'a>>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SerializeVec {
            enc: self.enc,
            vec: ArrayList::new(self.enc)?,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SerializeTupleVariant {
            enc: self.enc,
            name: variant.into(),
            vec: ArrayList::new(self.enc)?,
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.serialize_seq(len.map(|l| l * 2))
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(SerializeStructVariant {
            enc: self.enc,
            name: variant.into(),
            map: SerializeVec {
                enc: self.enc,
                vec: ArrayList::new(self.enc)?,
            },
        })
    }
}

pub struct SerializeVec<'a> {
    enc: &'a Encoder<'a>,
    vec: ArrayList<'a>,
}

pub struct SerializeTupleVariant<'a> {
    enc: &'a Encoder<'a>,
    name: String,
    vec: ArrayList<'a>,
}

pub struct SerializeStructVariant<'a> {
    enc: &'a Encoder<'a>,
    name: String,
    map: SerializeVec<'a>,
}

impl<'a> ser::SerializeSeq for SerializeVec<'a> {
    type Ok = JObject<'a>;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        let val = to_object(self.enc, &value)?;
        self.vec.add(val)
    }

    fn end(self) -> Result<JObject<'a>> {
        self.vec.to_vector()
    }
}

impl<'a> ser::SerializeTuple for SerializeVec<'a> {
    type Ok = JObject<'a>;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<JObject<'a>> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleStruct for SerializeVec<'a> {
    type Ok = JObject<'a>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<JObject<'a>> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleVariant for SerializeTupleVariant<'a> {
    type Ok = JObject<'a>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.vec.add(to_object(self.enc, &value)?)?;
        Ok(())
    }

    fn end(self) -> Result<JObject<'a>> {
        variant_map(self.enc, &self.name, self.vec.to_vector()?)
    }
}

impl<'a> ser::SerializeMap for SerializeVec<'a> {
    type Ok = JObject<'a>;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, key)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, &value)
    }

    fn end(self) -> Result<JObject<'a>> {
        self.vec.to_hashmap()
    }
}

impl<'a> ser::SerializeStruct for SerializeVec<'a> {
    type Ok = JObject<'a>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.vec.add(self.enc.get_keyword(key)?.into())?;
        ser::SerializeMap::serialize_value(self, value)
    }

    fn end(self) -> Result<JObject<'a>> {
        ser::SerializeMap::end(self)
    }
}

impl<'a> ser::SerializeStructVariant for SerializeStructVariant<'a> {
    type Ok = JObject<'a>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeStruct::serialize_field(&mut self.map, key, value)
    }

    fn end(self) -> Result<JObject<'a>> {
        variant_map(self.enc, &self.name, self.map.vec.to_hashmap()?)
    }
}
