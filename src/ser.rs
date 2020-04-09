use crate::error::{Error, Result};
use serde::{ser, Serialize};
use std::io::Write;

pub struct Serializer {
  output: Vec<u8>,
}

pub fn to_writer<T, W>(value: &T, writer: &mut W) -> Result<()>
where
  T: Serialize,
  W: Write,
{
  let mut serializer = Serializer { output: Vec::new() };
  value.serialize(&mut serializer)?;
  writer.write_all(&serializer.output)?;
  Ok(())
}

impl<'a> ser::Serializer for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  type SerializeSeq = Self;
  type SerializeTuple = Self;
  type SerializeTupleStruct = Self;
  type SerializeTupleVariant = Self;
  type SerializeMap = Self;
  type SerializeStruct = Self;
  type SerializeStructVariant = Self;

  fn serialize_bool(self, v: bool) -> Result<()> {
    if v {
      Ok(self.output.push(self.serialize_i64(1)?))
    } else {
      Ok(self.output.push(self.serialize_i64(0)?))
    }
  }

  fn serialize_i8(self, v: i8) -> Result<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i16(self, v: i16) -> Result<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i32(self, v: i32) -> Result<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i64(self, v: i64) -> Result<()> {
    self.output.push(b':');
    self.output.push(v.to_string().into_bytes());
    self.output.push(b"\r\n");
    Ok(())
  }

  fn serialize_u8(self, v: u8) -> Result<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_u16(self, v: u16) -> Result<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_u32(self, v: u32) -> Result<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_u64(self, v: u64) -> Result<()> {
    self.serialize_i64(i64::try_from(v)?)
  }

  fn serialize_f32(self, _: f32) -> Result<()> {
    Err(Error::UnsupportedType)
  }

  fn serialize_f64(self, _: f64) -> Result<()> {
    Err(Error::UnsupportedType)
  }

  fn serialize_char(self, v: char) -> Result<()> {
    // TODO: if char is \r or \n then need Bulk String, else Simple String
    Err(Error::UnsupportedType)
  }

  fn serialize_str(self, v: &str) -> Result<()> {
    // TODO: if contains \r or \n then need Bulk String, else Simple String
    Err(Error::UnsupportedType)
  }

  fn serialize_bytes(self, v: &[u8]) -> Result<()> {
    // TODO: if contains \r or \n then need Bulk String, else Simple String
    Err(Error::UnsupportedType)
  }

  fn serialize_none(self) -> Result<()> {
    self.output.push(b"$-1\r\n");
    Ok(())
  }

  fn serialize_some<T>(self, value: &T) -> Result<()>
  where
    T: Serialize + ?Sized,
  {
    value.serialize(self)
  }
}
