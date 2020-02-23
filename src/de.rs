use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};
use serde::Deserialize;

use jni::objects::{AutoLocal, JObject};
use jni::signature::{JavaType, Primitive};

use crate::convert::Decoder;
use crate::error::{Error, Result};

pub struct Deserializer<'de> {
    dec: &'de Decoder<'de>,
    obj: AutoLocal<'de, 'de>,
}

impl<'de> Deserializer<'de> {
    pub fn from_object(dec: &'de Decoder<'de>, obj: AutoLocal<'de, 'de>) -> Self {
        Deserializer { dec, obj }
    }
}

pub fn from_object<'a, T>(dec: &'a Decoder<'a>, obj: JObject<'a>) -> Result<T>
where
    T: Deserialize<'a>,
{
    let deserializer = Deserializer::from_object(dec, dec.com.env.auto_local(obj));
    T::deserialize(deserializer)
}

fn is_null<'a>(obj: JObject<'a>) -> bool {
    obj.clone().into_inner() == JObject::null().into_inner()
}

// based on https://serde.rs/impl-deserializer.html

impl<'de> de::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::DeserializeAnyNotSupported)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(b) = self.dec.decode_bool(self.obj.as_obj())? {
            visitor.visit_bool(b)
        } else {
            Err(Error::ExpectedBoolean)
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.dec.decode_i8(self.obj.as_obj())? {
            visitor.visit_i8(v)
        } else if let Some(v) = self.dec.decode_i16(self.obj.as_obj())? {
            visitor.visit_i16(v)
        } else if let Some(v) = self.dec.decode_i32(self.obj.as_obj())? {
            visitor.visit_i32(v)
        } else if let Some(v) = self.dec.decode_i64(self.obj.as_obj())? {
            visitor.visit_i64(v)
        } else {
            Err(Error::ExpectedInteger)
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.dec.decode_i16(self.obj.as_obj())? {
            visitor.visit_i16(v)
        } else if let Some(v) = self.dec.decode_i8(self.obj.as_obj())? {
            visitor.visit_i8(v)
        } else if let Some(v) = self.dec.decode_i32(self.obj.as_obj())? {
            visitor.visit_i32(v)
        } else if let Some(v) = self.dec.decode_i64(self.obj.as_obj())? {
            visitor.visit_i64(v)
        } else {
            Err(Error::ExpectedInteger)
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.dec.decode_i32(self.obj.as_obj())? {
            visitor.visit_i32(v)
        } else if let Some(v) = self.dec.decode_i8(self.obj.as_obj())? {
            visitor.visit_i8(v)
        } else if let Some(v) = self.dec.decode_i16(self.obj.as_obj())? {
            visitor.visit_i16(v)
        } else if let Some(v) = self.dec.decode_i64(self.obj.as_obj())? {
            visitor.visit_i64(v)
        } else {
            Err(Error::ExpectedInteger)
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(v) = self.dec.decode_i64(self.obj.as_obj())? {
            visitor.visit_i64(v)
        } else if let Some(v) = self.dec.decode_i8(self.obj.as_obj())? {
            visitor.visit_i8(v)
        } else if let Some(v) = self.dec.decode_i16(self.obj.as_obj())? {
            visitor.visit_i16(v)
        } else if let Some(v) = self.dec.decode_i32(self.obj.as_obj())? {
            visitor.visit_i32(v)
        } else {
            Err(Error::ExpectedInteger)
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Deserializer::deserialize_i8(self, visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Deserializer::deserialize_i16(self, visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Deserializer::deserialize_i32(self, visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Deserializer::deserialize_i64(self, visitor)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(f) = self.dec.decode_f32(self.obj.as_obj())? {
            visitor.visit_f32(f)
        } else if let Some(f) = self.dec.decode_f64(self.obj.as_obj())? {
            visitor.visit_f64(f)
        } else {
            Err(Error::ExpectedFloat)
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(f) = self.dec.decode_f64(self.obj.as_obj())? {
            visitor.visit_f64(f)
        } else if let Some(f) = self.dec.decode_f32(self.obj.as_obj())? {
            visitor.visit_f32(f)
        } else {
            Err(Error::ExpectedFloat)
        }
    }

    // TODO
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(s) = self.dec.decode_string(self.obj.as_obj())? {
            if s.len() == 1 {
                return visitor.visit_char(s.chars().next().unwrap());
            }
        }
        Err(Error::ExpectedChar)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Deserializer::deserialize_string(self, visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(s) = self.dec.decode_string(self.obj.as_obj())? {
            visitor.visit_string(s)
        } else if let Some(s) = self.dec.decode_keyword(self.obj.as_obj())? {
            visitor.visit_string(s)
        } else {
            Err(Error::ExpectedString)
        }
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if is_null(self.obj.as_obj()) {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if is_null(self.obj.as_obj()) {
            visitor.visit_unit()
        } else {
            Err(Error::ExpectedNull)
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(Seq {
            dec: self.dec,
            seq: self.obj,
        })
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some((key_iter, val_iter)) = self.dec.map_to_iters(self.obj)? {
            visitor.visit_map(Map {
                dec: self.dec,
                key_iter,
                val_iter,
            })
        } else {
            Err(Error::ExpectedMap)
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // test if it's a bare keyword (unit variant)
        if let Some(s) = self.dec.decode_keyword(self.obj.as_obj())? {
            visitor.visit_enum(s.into_deserializer())
        } else if let Some((key_iter, val_iter)) = self.dec.map_to_iters(self.obj)? {
            visitor.visit_enum(Map {
                dec: self.dec,
                key_iter,
                val_iter,
            })
        } else {
            Err(Error::ExpectedMap)
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct Seq<'de> {
    dec: &'de Decoder<'de>,
    seq: AutoLocal<'de, 'de>,
}

impl<'de, 'a> SeqAccess<'de> for Seq<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        let first = self.dec.com.env.auto_local(
            self.dec
                .com
                .env
                .call_static_method_unchecked(
                    self.dec.class_rt,
                    self.dec.first_seq,
                    JavaType::Object(String::new()),
                    &[self.seq.as_obj().into()],
                )?
                .l()?,
        );

        if is_null(first.as_obj()) {
            return Ok(None);
        }
        self.seq = self.dec.com.env.auto_local(
            self.dec
                .com
                .env
                .call_static_method_unchecked(
                    self.dec.class_rt,
                    self.dec.next_seq,
                    JavaType::Object(String::new()),
                    &[self.seq.as_obj().into()],
                )?
                .l()?,
        );
        Ok(Some(
            seed.deserialize(Deserializer::from_object(self.dec, first))?,
        ))
    }
}

struct Map<'de> {
    dec: &'de Decoder<'de>,
    key_iter: AutoLocal<'de, 'de>,
    val_iter: AutoLocal<'de, 'de>,
}

impl<'de> Map<'de> {
    fn next_key(&self) -> Result<Option<AutoLocal<'de, 'de>>> {
        if !self
            .dec
            .com
            .env
            .call_method_unchecked(
                self.key_iter.as_obj(),
                self.dec.hasnext_iter,
                JavaType::Primitive(Primitive::Boolean),
                &[],
            )?
            .z()?
        {
            return Ok(None);
        }
        let key = self.dec.com.env.auto_local(
            self.dec
                .com
                .env
                .call_method_unchecked(
                    self.key_iter.as_obj(),
                    self.dec.next_iter,
                    JavaType::Object(String::new()),
                    &[],
                )?
                .l()?,
        );
        Ok(Some(key))
    }
    fn next_val(&self) -> Result<AutoLocal<'de, 'de>> {
        Ok(self.dec.com.env.auto_local(
            self.dec
                .com
                .env
                .call_method_unchecked(
                    self.val_iter.as_obj(),
                    self.dec.next_iter,
                    JavaType::Object(String::new()),
                    &[],
                )?
                .l()?,
        ))
    }
}

impl<'de, 'a> MapAccess<'de> for Map<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some(key) = Map::next_key(&self)? {
            Ok(Some(
                seed.deserialize(Deserializer::from_object(self.dec, key))?,
            ))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        Ok(seed.deserialize(Deserializer::from_object(self.dec, Map::next_val(&self)?))?)
    }
}

impl<'de> EnumAccess<'de> for Map<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(key) = MapAccess::next_key_seed(&mut self, seed)? {
            Ok((key, self))
        } else {
            Err(Error::ExpectedMap)
        }
    }
}

impl<'de> VariantAccess<'de> for Map<'de> {
    type Error = Error;
    fn unit_variant(self) -> Result<()> {
        // this shouldn't happen because we handled it in deserialize_enum
        Err(Error::ExpectedKeyword)
    }
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(Deserializer::from_object(self.dec, Map::next_val(&self)?))
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(
            Deserializer::from_object(self.dec, Map::next_val(&self)?),
            visitor,
        )
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_map(
            Deserializer::from_object(self.dec, Map::next_val(&self)?),
            visitor,
        )
    }
}
