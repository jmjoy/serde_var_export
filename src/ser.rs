use serde::{ser, Serialize};

use crate::error::{Error, Result};
use std::fmt::Display;
use std::io::Write;

#[doc(hidden)]
pub struct SeqSerializer<'a, W: Write> {
    ser: &'a mut Serializer<W>,
    idx: usize,
}

impl<'a, W: Write> SeqSerializer<'a, W> {
    #[inline]
    fn new(ser: &'a mut Serializer<W>) -> Self {
        Self { ser, idx: 0 }
    }

    #[inline]
    fn as_mut_writer(&mut self) -> &mut W {
        &mut self.ser.writer
    }
}

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
    type SerializeSeq = SeqSerializer<'a, W>;
    type SerializeTuple = SeqSerializer<'a, W>;
    type SerializeTupleStruct = SeqSerializer<'a, W>;
    type SerializeTupleVariant = SeqSerializer<'a, W>;
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
        dtoa::write(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        dtoa::write(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        let mut s = [0u8; 4];
        self.serialize_str(v.encode_utf8(&mut s));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        let mut s = String::new();
        s.push('\'');
        for c in v.chars() {
            if c == '\'' || c == '\\' {
                s.push('\\');
            }
            s.push(c);
        }
        s.push('\'');
        self.writer.write_all(s.as_bytes())?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.writer.write_all(b"NULL")?;
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.writer.write_all(b"NULL")?;
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        self.writer.write_all(b"NULL")?;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)?;
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(self)
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
        self.writer.write_all(b"array(\n  ")?;
        variant.serialize(&mut *self)?;
        self.writer.write_all(b" => ")?;
        value.serialize(&mut *self)?;
        self.writer.write_all(b",\n)")?;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.writer.write_all(b"array(\n")?;
        Ok(SeqSerializer::new(self))
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
        self.writer.write_all(b"array(\n  ")?;
        variant.serialize(&mut *self)?;
        self.writer.write_all(b" => array(\n")?;
        Ok(SeqSerializer::new(self))
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

impl<'a, W: Write> ser::SerializeSeq for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.as_mut_writer().write_all(b"  ")?;
        (self.idx as u64).serialize(&mut *self.ser)?;
        self.as_mut_writer().write_all(b" => ")?;
        value.serialize(&mut *self.ser)?;
        self.as_mut_writer().write_all(b",\n")?;
        self.idx += 1;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok> {
        self.as_mut_writer().write_all(b")")?;
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTuple for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W: Write> ser::SerializeTupleStruct for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W: Write> ser::SerializeTupleVariant for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(mut self) -> Result<Self::Ok> {
        self.as_mut_writer().write_all(b"))")?;
        Ok(())
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
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_bool(true)),
            "true"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_bool(false)),
            "false"
        );
    }

    #[test]
    fn serialize_i8() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_i8(0)),
            "0"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_i8(127)),
            "127"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_i8(-127)),
            "-127"
        );
    }

    #[test]
    fn serialize_i16() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_i16(0)),
            "0"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_i16(32767)),
            "32767"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_i16(-32767)),
            "-32767"
        );
    }

    #[test]
    fn serialize_i32() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_i32(0)),
            "0"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_i32(2147483647)),
            "2147483647"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_i32(-2147483647)),
            "-2147483647"
        );
    }

    #[test]
    fn serialize_i64() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_i64(0)),
            "0"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_i64(2147483647)),
            "2147483647"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_i64(-2147483647)),
            "-2147483647"
        );
    }

    #[test]
    fn serialize_u8() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_u8(0)),
            "0"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_u8(127)),
            "127"
        );
    }

    #[test]
    fn serialize_u16() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_u16(0)),
            "0"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_u16(32767)),
            "32767"
        );
    }

    #[test]
    fn serialize_u32() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_u32(0)),
            "0"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_u32(2147483647)),
            "2147483647"
        );
    }

    #[test]
    fn serialize_u64() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_u64(0)),
            "0"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_u64(2147483647)),
            "2147483647"
        );
    }

    #[test]
    fn serialize_f32() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_f32(0.0)),
            "0.0"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_f32(1.0)),
            "1.0"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_f32(1.01)),
            "1.01"
        );
    }

    #[test]
    fn serialize_f64() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_f64(0.0)),
            "0.0"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_f64(1.0)),
            "1.0"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_f64(1.01)),
            "1.01"
        );
    }

    #[test]
    fn serialize_char() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_char('a')),
            "'a'"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_char('\n')),
            "'\n'"
        );
    }

    #[test]
    fn serialize_str() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_str("foo")),
            "'foo'"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_str("\r\n")),
            "'\r\n'"
        );
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_str(r#""'bar'""#)),
            r#"'"\'bar\'"'"#
        );
    }

    #[test]
    fn serialize_bytes() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_bytes(b"foo")),
            "array(
  0 => 102,
  1 => 111,
  2 => 111,
)"
        );
    }

    #[test]
    fn serialize_none() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_none()),
            "NULL"
        );
    }

    #[test]
    fn serialize_some() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_some(&1i32)),
            "1"
        );

        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_some("foo")),
            "'foo'"
        );
    }

    #[test]
    fn serialize_unit() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_unit()),
            "NULL"
        );
    }

    #[test]
    fn serialize_unit_struct() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_unit_struct("Foo")),
            "NULL"
        );
    }

    #[test]
    fn serialize_unit_variant() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_unit_variant("Foo", 0, "Bar")),
            "'Bar'"
        );
    }

    #[test]
    fn serialize_newtype_struct() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_newtype_struct("Foo", &1i32)),
            "1"
        );

        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_newtype_struct("Foo", "foo")),
            "'foo'"
        );
    }

    #[test]
    fn serialize_newtype_variant() {
        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_newtype_variant("Foo", 0, "Bar", &1i32)),
            "array(
  'Bar' => 1,
)"
        );

        assert_eq!(
            &serialize_to_string(|serializer| serializer.serialize_newtype_variant("Foo", 0, "Bar", "bar")),
            "array(
  'Bar' => 'bar',
)"
        );
    }

    #[test]
    fn serialize_seq() {
        let mut serializer = Serializer::new(Vec::new());
        let mut seq_serializer = serializer.serialize_seq(None).unwrap();
        assert_eq!(seq_serializer.as_mut_writer(), b"array(\n");
        assert_eq!(seq_serializer.idx, 0);

        let ptr = seq_serializer.ser as *const Serializer<Vec<u8>>;
        drop(seq_serializer);
        assert_eq!(ptr, &serializer as *const Serializer<Vec<u8>>);
    }

    #[test]
    fn serialize_tuple() {
        let mut serializer = Serializer::new(Vec::new());
        let mut tuple_serializer = serializer.serialize_tuple(0).unwrap();
        assert_eq!(tuple_serializer.as_mut_writer(), b"array(\n");
        assert_eq!(tuple_serializer.idx, 0);

        let ptr = tuple_serializer.ser as *const Serializer<Vec<u8>>;
        drop(tuple_serializer);
        assert_eq!(ptr, &serializer as *const Serializer<Vec<u8>>);
    }

    #[test]
    fn serialize_tuple_struct() {
        let mut serializer = Serializer::new(Vec::new());
        let mut tuple_struct_serializer = serializer.serialize_tuple_struct("Foo", 0).unwrap();
        assert_eq!(tuple_struct_serializer.as_mut_writer(), b"array(\n");
        assert_eq!(tuple_struct_serializer.idx, 0);

        let ptr = tuple_struct_serializer.ser as *const Serializer<Vec<u8>>;
        drop(tuple_struct_serializer);
        assert_eq!(ptr, &serializer as *const Serializer<Vec<u8>>);
    }

    #[test]
    #[ignore]
    fn serialize_tuple_variant() {
        let mut serializer = Serializer::new(Vec::new());
        serializer.serialize_tuple_variant("Foo", 0, "Bar", 0);
    }

    fn serialize_to_string<F>(f: F) -> String
    where
        F: Fn(&mut Serializer<Vec<u8>>) -> Result<()>,
    {
        let mut serializer = Serializer::new(Vec::new());
        f(&mut serializer).unwrap();
        String::from_utf8(serializer.writer).unwrap()
    }
}
