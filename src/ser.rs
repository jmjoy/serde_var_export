use serde::{ser, Serialize};

use crate::error::{Error, Result};
use std::io::Write;

const INDENT: &'static [u8] = b"  ";

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer::new(Vec::new());
    value.serialize(&mut serializer)?;
    Ok(String::from_utf8(serializer.writer)?)
}

pub fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = Serializer::new(Vec::new());
    value.serialize(&mut serializer)?;
    Ok(serializer.writer)
}

pub fn to_writer<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    W: Write,
    T: Serialize,
{
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)?;
    Ok(())
}

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
    current_ident: usize,
}

impl<W: Write> Serializer<W> {
    #[inline]
    fn new(writer: W) -> Self {
        Self {
            writer,
            current_ident: 0,
        }
    }

    fn write_begin_array(&mut self) -> Result<()> {
        if self.current_ident > 0 {
            self.writer.write_all(b"\n")?;
            self.write_ident()?;
        }
        self.writer.write_all(b"array(\n")?;
        self.current_ident += 1;
        Ok(())
    }

    fn write_end_array(&mut self) -> Result<()> {
        self.current_ident -= 1;
        self.write_ident()?;
        self.writer.write_all(b")")?;
        Ok(())
    }

    #[inline]
    fn write_ident(&mut self) -> Result<()> {
        for _ in 0..self.current_ident {
            self.writer.write_all(INDENT)?;
        }
        Ok(())
    }

    fn write_map_symbol(&mut self) -> Result<()> {
        self.writer.write_all(b" => ")?;
        Ok(())
    }
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
        self.serialize_str(v.encode_utf8(&mut s))?;
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

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
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
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        self.write_begin_array()?;
        self.write_ident()?;
        variant.serialize(&mut *self)?;
        self.write_map_symbol()?;
        value.serialize(&mut *self)?;
        self.writer.write_all(b",\n")?;
        self.write_end_array()?;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.write_begin_array()?;
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
        self.write_begin_array()?;
        self.write_ident()?;
        variant.serialize(&mut *self)?;
        self.writer.write_all(b" => ")?;
        self.write_begin_array()?;
        Ok(SeqSerializer::new(self))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.write_begin_array()?;
        Ok(self)
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
        self.write_begin_array()?;
        self.write_ident()?;
        variant.serialize(&mut *self)?;
        self.writer.write_all(b" => ")?;
        self.write_begin_array()?;
        Ok(self)
    }
}

impl<'a, W: Write> ser::SerializeSeq for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.ser.write_ident()?;
        (self.idx as u64).serialize(&mut *self.ser)?;
        self.ser.write_map_symbol()?;
        value.serialize(&mut *self.ser)?;
        self.as_mut_writer().write_all(b",\n")?;
        self.idx += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.ser.write_end_array()?;
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
        self.ser.write_end_array()?;
        self.as_mut_writer().write_all(b",\n")?;
        self.ser.write_end_array()?;
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
        self.write_ident()?;
        key.serialize(&mut **self)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.write_map_symbol()?;
        value.serialize(&mut **self)?;
        self.writer.write_all(b",\n")?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.write_end_array()?;
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeMap::serialize_key(self, key)?;
        ser::SerializeMap::serialize_value(self, value)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeMap::end(self)
    }
}

impl<'a, W: Write> ser::SerializeStructVariant for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok> {
        self.write_end_array()?;
        self.writer.write_all(b",\n")?;
        self.write_end_array()?;
        Ok(())
    }
}

#[cfg(test)]
mod new_tests {
    use super::*;
    use crate::ser::SeqSerializer;

    #[test]
    fn new_serializer() {
        let mut vec = Vec::new();
        let serializer = Serializer::new(&mut vec);
        assert_eq!(serializer.current_ident, 0);
        assert_eq!(serializer.writer.as_ptr(), vec.as_ptr());
    }

    #[test]
    fn new_seq_serializer() {
        let mut vec = Vec::new();
        let mut serializer = Serializer::new(&mut vec);
        let mut seq_serializer = SeqSerializer::new(&mut serializer);
        assert_eq!(seq_serializer.idx, 0);
        assert_eq!(
            seq_serializer.as_mut_writer().as_ptr(),
            serializer.writer.as_ptr()
        );
    }
}

#[cfg(test)]
mod serializer_tests {
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
            &serialize_to_string(
                |serializer| serializer.serialize_newtype_variant("Foo", 0, "Bar", &1i32)
            ),
            "array(
  'Bar' => 1,
)"
        );

        assert_eq!(
            &serialize_to_string(
                |serializer| serializer.serialize_newtype_variant("Foo", 0, "Bar", "bar")
            ),
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
    fn serialize_tuple_variant() {
        let mut serializer = Serializer::new(Vec::new());
        let mut tuple_variant_serializer = serializer
            .serialize_tuple_variant("Foo", 0, "Bar", 0)
            .unwrap();

        assert_eq!(
            std::str::from_utf8(tuple_variant_serializer.as_mut_writer()).unwrap(),
            "array(\n  'Bar' => \n  array(\n"
        );
        assert_eq!(tuple_variant_serializer.idx, 0);

        let ptr = tuple_variant_serializer.ser as *const Serializer<Vec<u8>>;
        drop(tuple_variant_serializer);
        assert_eq!(ptr, &serializer as *const Serializer<Vec<u8>>);
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

#[cfg(test)]
mod to_xyz_tests {
    #[test]
    fn to_string() {
        assert_eq!(super::to_string(&true).unwrap(), "true");
    }

    #[test]
    fn to_vec() {
        assert_eq!(super::to_vec(&true).unwrap(), b"true");
    }

    #[test]
    fn to_writer() {
        let mut buf = Vec::new();
        super::to_writer(&mut buf, &true).unwrap();
        assert_eq!(buf, b"true");
    }
}
