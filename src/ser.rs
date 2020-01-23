use serde::{ser, Serialize};

use crate::error::{Error, Result};
use std::fmt::Display;
use std::io::Write;

pub struct Serializer<W: Write> {
    writer: W,
}

impl<W: Write> Serializer<W> {
    fn new(writer: W) -> Self {
        Self { writer }
    }
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer::new(Vec::new());
    value.serialize(&mut serializer)?;
    Ok(String::from_utf8(serializer.writer)?)
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.writer.write_all(if v { b"true" } else { b"false" })?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.writer.write_all(v.to_string().as_bytes())?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.writer.write_all(v.to_string().as_bytes())?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.writer.write_all(format!("{:?}", v).as_bytes())?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.writer.write_all(format!("{:?}", v).as_bytes())?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.writer.write_all(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.writer.write_all(b"null")?;
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.writer.write_all(b"null")?;
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.writer.write_all(b"null")?;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Display,
    {
        unimplemented!()
    }
}

impl<'a, W: Write> ser::SerializeSeq for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a, W: Write> ser::SerializeTuple for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a, W: Write> ser::SerializeTupleStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a, W: Write> ser::SerializeTupleVariant for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a, W: Write> ser::SerializeMap for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a, W: Write> ser::SerializeStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a, W: Write> ser::SerializeStructVariant for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::ser::Serializer as _;

    #[test]
    fn serialize_bool() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_bool(true).unwrap();
        assert_eq!(&serializer.writer, b"true");

        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_bool(false).unwrap();
        assert_eq!(&serializer.writer, b"false");
    }

    #[test]
    fn serialize_i8() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_i8(127).unwrap();
        assert_eq!(&serializer.writer, b"127");
    }

    #[test]
    fn serialize_i16() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_i16(32767).unwrap();
        assert_eq!(&serializer.writer, b"32767");
    }

    #[test]
    fn serialize_i32() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_i32(2147483647).unwrap();
        assert_eq!(&serializer.writer, b"2147483647");
    }

    #[test]
    fn serialize_i64() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_i64(2147483647).unwrap();
        assert_eq!(&serializer.writer, b"2147483647");
    }

    #[test]
    fn serialize_u8() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_u8(127).unwrap();
        assert_eq!(&serializer.writer, b"127");
    }

    #[test]
    fn serialize_u16() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_u16(32767).unwrap();
        assert_eq!(&serializer.writer, b"32767");
    }

    #[test]
    fn serialize_u32() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_u32(2147483647).unwrap();
        assert_eq!(&serializer.writer, b"2147483647");
    }

    #[test]
    fn serialize_u64() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_u64(2147483647).unwrap();
        assert_eq!(&serializer.writer, b"2147483647");
    }

    #[test]
    fn serialize_f32() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_f32(1.0).unwrap();
        assert_eq!(&serializer.writer, b"1");

        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_f32(std::f32::INFINITY).unwrap();
        assert_eq!(&serializer.writer, b"inf");
    }

    #[test]
    fn serialize_f64() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_f64(1.01).unwrap();
        assert_eq!(&serializer.writer, b"1.01");
    }

    #[test]
    fn serialize_char() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_char('a').unwrap();
        assert_eq!(&serializer.writer, br"'a'");

        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_char('\n').unwrap();
        assert_eq!(&serializer.writer, br"'\n'");
    }

    #[test]
    fn serialize_str() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_str("hello").unwrap();
        assert_eq!(&serializer.writer, br#""hello""#);

        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_str("\r\n").unwrap();
        assert_eq!(&serializer.writer, br#""\r\n""#);

        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_str(r#""hello 'world'""#).unwrap();
        assert_eq!(&serializer.writer, br#""\"hello \'world\'\"""#);
    }

    #[test]
    fn serialize_bytes() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_bytes(b"???").unwrap();
        assert_eq!(&serializer.writer, b"???");
    }

    #[test]
    fn serialize_none() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_none().unwrap();
        assert_eq!(&serializer.writer, b"null");
    }

    #[test]
    fn serialize_some() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_none().unwrap();
        assert_eq!(&serializer.writer, b"null");
    }
}
