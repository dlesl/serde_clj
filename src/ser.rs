use jni::objects::{JObject, JValue};
use jni::sys::jboolean;
use serde::{self, ser, Serialize};

use crate::convert::{ArrayList, Encoder};
use crate::error::{Error, Result};

pub struct Serializer<'a> {
    enc: &'a Encoder<'a>,
}

impl<'a> Serializer<'a> {}

// based on https://github.com/serde-rs/json/blob/master/src/value/ser.rs

pub fn to_value<'a, T>(enc: &'a Encoder<'a>, value: &T) -> Result<JValue<'a>>
where
    T: Serialize,
{
    let serializer = Serializer { enc };
    value.serialize(serializer)
}

pub fn to_object<'a, T>(enc: &'a Encoder<'a>, value: &T) -> Result<JObject<'a>>
where
    T: Serialize,
{
    to_value(enc.clone(), value).and_then(|v| enc.to_boxed(v))
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
    type Ok = JValue<'a>;
    type Error = Error;

    type SerializeSeq = SerializeVec<'a>;
    type SerializeTuple = SerializeVec<'a>;
    type SerializeTupleStruct = SerializeVec<'a>;
    type SerializeTupleVariant = SerializeTupleVariant<'a>;
    type SerializeMap = SerializeVec<'a>;
    type SerializeStruct = SerializeVec<'a>;
    type SerializeStructVariant = SerializeStructVariant<'a>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<JValue<'a>> {
        Ok(JValue::Bool(value as jboolean))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<JValue<'a>> {
        Ok(JValue::Byte(value))
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<JValue<'a>> {
        Ok(JValue::Short(value))
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<JValue<'a>> {
        Ok(JValue::Int(value))
    }

    fn serialize_i64(self, value: i64) -> Result<JValue<'a>> {
        Ok(JValue::Long(value))
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<JValue<'a>> {
        Ok(JValue::Short(value as i16))
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<JValue<'a>> {
        Ok(JValue::Int(value as i32))
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<JValue<'a>> {
        Ok(JValue::Long(value as i64))
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<JValue<'a>> {
        Ok(JValue::Long(value as i64)) // FIXME: is this OK? Java
                                       // doesn't have unsigned types but does let you treat them as unsigned...
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<JValue<'a>> {
        Ok(JValue::Float(value))
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<JValue<'a>> {
        Ok(JValue::Double(value))
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<JValue<'a>> {
        // Rust's chars can be 32 bit so we need to encode as string
        let mut s = String::new();
        s.push(value);
        self.serialize_str(&s)
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<JValue<'a>> {
        Ok(JValue::Object(self.enc.com.env.new_string(value)?.into()).into())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<JValue<'a>> {
        Ok(JObject::from(self.enc.com.env.byte_array_from_slice(value)?).into())
    }

    #[inline]
    fn serialize_unit(self) -> Result<JValue<'a>> {
        Ok(JValue::Object(JObject::null()))
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<JValue<'a>> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<JValue<'a>> {
        // just a bare keyword
        Ok(self.enc.get_keyword(variant)?.into())
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<JValue<'a>>
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
    ) -> Result<JValue<'a>>
    where
        T: Serialize,
    {
        Ok(variant_map(self.enc, variant, to_object(self.enc, &value)?)?.into())
    }

    #[inline]
    fn serialize_none(self) -> Result<JValue<'a>> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<JValue<'a>>
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
    type Ok = JValue<'a>;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        let val = to_object(self.enc, &value)?;
        self.vec.add(val)
    }

    fn end(self) -> Result<JValue<'a>> {
        Ok(self.vec.to_vector()?.into())
    }
}

impl<'a> ser::SerializeTuple for SerializeVec<'a> {
    type Ok = JValue<'a>;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<JValue<'a>> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleStruct for SerializeVec<'a> {
    type Ok = JValue<'a>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<JValue<'a>> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleVariant for SerializeTupleVariant<'a> {
    type Ok = JValue<'a>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.vec.add(to_object(self.enc, &value)?)?;
        Ok(())
    }

    fn end(self) -> Result<JValue<'a>> {
        Ok(variant_map(self.enc, &self.name, self.vec.to_vector()?)?.into())
    }
}

impl<'a> ser::SerializeMap for SerializeVec<'a> {
    type Ok = JValue<'a>;
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

    fn end(self) -> Result<JValue<'a>> {
        Ok(self.vec.to_hashmap()?.into())
    }
}

impl<'a> ser::SerializeStruct for SerializeVec<'a> {
    type Ok = JValue<'a>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.vec.add(self.enc.get_keyword(key)?.into())?;
        ser::SerializeMap::serialize_value(self, value)
    }

    fn end(self) -> Result<JValue<'a>> {
        ser::SerializeMap::end(self)
    }
}

impl<'a> ser::SerializeStructVariant for SerializeStructVariant<'a> {
    type Ok = JValue<'a>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeStruct::serialize_field(&mut self.map, key, value)
    }

    fn end(self) -> Result<JValue<'a>> {
        Ok(variant_map(self.enc, &self.name, self.map.vec.to_hashmap()?)?.into())
    }
}
