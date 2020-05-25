use crate::error::{Error, Result};
use serde::{ser, Serialize};
use std::io::Write;

struct Serializer {
  command: bool,
  output: Vec<u8>,
}

pub fn write_command<T, W>(value: &T, writer: &mut W) -> Result<()>
where
  T: Serialize,
  W: Write,
{
  let mut serializer = Serializer {
    command: true,
    output: Vec::new(),
  };
  value.serialize(&mut serializer)?;
  writer.write_all(&serializer.output)?;
  writer.write_all(b"\r\n")?;
  Ok(())
}

pub fn write_reply<T, W>(value: &T, writer: &mut W) -> Result<()>
where
  T: Serialize,
  W: Write,
{
  let mut serializer = Serializer {
    command: false,
    output: Vec::new(),
  };
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
      self.serialize_i64(1)
    } else {
      self.serialize_i64(0)
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
    self.output.write_all(&v.to_string().into_bytes())?;
    self.output.write_all(b"\r\n")?;
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
    use std::convert::TryFrom;
    self.serialize_i64(i64::try_from(v)?)
  }

  fn serialize_f32(self, _: f32) -> Result<()> {
    Err(Error::UnsupportedType)
  }

  fn serialize_f64(self, _: f64) -> Result<()> {
    Err(Error::UnsupportedType)
  }

  fn serialize_char(self, v: char) -> Result<()> {
    if !v.is_ascii() {
      return Err(Error::UnsupportedType);
    }

    if v == '\r' || v == '\n' {
      self.serialize_bytes(&[v as u8])
    } else {
      self.output.push(b'+');
      self.output.push(v as u8);
      self.output.write_all(b"\r\n")?;

      Ok(())
    }
  }

  fn serialize_str(self, v: &str) -> Result<()> {
    if v.contains('\r') || v.contains('\n') {
      self.serialize_bytes(v.as_bytes())
    } else {
      self.output.push(b'+');
      self.output.write_all(v.as_bytes())?;
      self.output.write_all(b"\r\n")?;

      Ok(())
    }
  }

  fn serialize_bytes(self, v: &[u8]) -> Result<()> {
    // bytes are always encoded in Bulk String
    self.output.push(b'$');
    self.output.write_all(v.len().to_string().as_bytes())?;
    self.output.write_all(b"\r\n")?;
    self.output.write_all(v)?;
    self.output.write_all(b"\r\n")?;
    Ok(())
  }

  fn serialize_none(self) -> Result<()> {
    // for command, omit optional arguments
    if !self.command {
      self.output.write_all(b"$-1\r\n")?;
    }
    Ok(())
  }

  fn serialize_some<T>(self, value: &T) -> Result<()>
  where
    T: Serialize + ?Sized,
  {
    value.serialize(self)
  }

  fn serialize_unit(self) -> Result<()> {
    self.output.write_all(b"$-1\r\n")?;
    Ok(())
  }

  fn serialize_unit_struct(self, _: &'static str) -> Result<()> {
    self.output.write_all(b"$-1\r\n")?;
    Ok(())
  }

  fn serialize_unit_variant(self, _: &'static str, _: u32, variant: &'static str) -> Result<()> {
    if !self.command {
      return Err(Error::InvalidTypeForReply);
    }

    self.output.write_all(b"*1\r\n")?;

    self.output.push(b'$');
    self.output.write_all(variant.len().to_string().as_bytes())?;
    self.output.write_all(b"\r\n")?;

    let cmd = variant.to_ascii_uppercase();
    self.output.write_all(cmd.as_bytes())?;
    self.output.write_all(b"\r\n")?;

    Ok(())
  }

  fn serialize_newtype_struct<T>(self, _: &'static str, value: &T) -> Result<()>
  where
    T: Serialize + ?Sized,
  {
    value.serialize(self)
  }

  fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, variant: &'static str, value: &T) -> Result<()>
  where
    T: Serialize + ?Sized,
  {
    if !self.command {
      return Err(Error::InvalidTypeForReply);
    }

    self.output.write_all(b"*2\r\n")?;

    self.output.push(b'$');
    self.output.write_all(variant.len().to_string().as_bytes())?;
    self.output.write_all(b"\r\n")?;

    let cmd = variant.to_ascii_uppercase();
    self.output.write_all(cmd.as_bytes())?;
    self.output.write_all(b"\r\n")?;

    value.serialize(self)
  }

  fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
    if self.command {
      // TODO: command Array size will need fixing
      Ok(self)
    } else {
      if let Some(len) = len {
        self.output.push(b'*');
        self.output.write_all(len.to_string().as_bytes())?;
        self.output.write_all(b"\r\n")?;

        Ok(self)
      } else {
        Err(Error::UnknownSize)
      }
    }
  }

  fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
    if self.command {
      // TODO: command Array size will need fixing
      Ok(self)
    } else {
      self.output.push(b'*');
      self.output.write_all(len.to_string().as_bytes())?;
      self.output.write_all(b"\r\n")?;

      Ok(self)
    }
  }

  fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct> {
    Err(Error::UnsupportedType)
  }

  fn serialize_tuple_variant(
    self,
    _: &'static str,
    _: u32,
    variant: &'static str,
    len: usize,
  ) -> Result<Self::SerializeTupleVariant> {
    if self.command {
      // TODO: command Array size will need fixing
      // write Array size (len + 1)
      self.output.push(b'*');
      self.output.write_all((len + 1).to_string().as_bytes())?;
      self.output.write_all(b"\r\n")?;

      // write command
      self.output.push(b'$');
      self.output.write_all(variant.len().to_string().as_bytes())?;
      self.output.write_all(b"\r\n")?;

      let cmd = variant.to_ascii_uppercase();
      self.output.write_all(cmd.as_bytes())?;
      self.output.write_all(b"\r\n")?;

      // write args
      Ok(self)
    } else {
      self.output.push(b'*');
      self.output.write_all(len.to_string().as_bytes())?;
      self.output.write_all(b"\r\n")?;

      Ok(self)
    }
  }

  fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap> {
    Err(Error::UnsupportedType)
  }

  fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct> {
    Err(Error::UnsupportedType)
  }

  fn serialize_struct_variant(
    self,
    _: &'static str,
    _: u32,
    _: &'static str,
    _: usize,
  ) -> Result<Self::SerializeStructVariant> {
    Err(Error::UnsupportedType)
  }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  fn serialize_element<T>(&mut self, value: &T) -> Result<()>
  where
    T: Serialize + ?Sized,
  {
    value.serialize(&mut **self)
  }

  fn end(self) -> Result<Self::Ok> {
    Ok(())
  }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
  where
    T: Serialize,
  {
    value.serialize(&mut **self)
  }

  fn end(self) -> Result<Self::Ok> {
    Ok(())
  }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<()>
  where
    T: Serialize,
  {
    Err(Error::UnsupportedType)
  }

  fn end(self) -> Result<Self::Ok> {
    Ok(())
  }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
  where
    T: Serialize,
  {
    value.serialize(&mut **self)
  }

  fn end(self) -> Result<Self::Ok> {
    Ok(())
  }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  fn serialize_key<T: ?Sized>(&mut self, _: &T) -> Result<()>
  where
    T: Serialize,
  {
    Err(Error::UnsupportedType)
  }

  fn serialize_value<T: ?Sized>(&mut self, _: &T) -> Result<()>
  where
    T: Serialize,
  {
    Err(Error::UnsupportedType)
  }

  fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, _: &K, _: &V) -> Result<()>
  where
    K: Serialize,
    V: Serialize,
  {
    Err(Error::UnsupportedType)
  }

  fn end(self) -> Result<Self::Ok> {
    Ok(())
  }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T: ?Sized>(&mut self, _: &'static str, _: &T) -> Result<()>
  where
    T: Serialize,
  {
    Err(Error::UnsupportedType)
  }

  fn end(self) -> Result<Self::Ok> {
    Ok(())
  }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T: ?Sized>(&mut self, _: &'static str, _: &T) -> Result<()>
  where
    T: Serialize,
  {
    Err(Error::UnsupportedType)
  }

  fn end(self) -> Result<Self::Ok> {
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use serde::Serialize;

  #[derive(Debug, Serialize)]
  enum Command {
    Ping,
  }

  #[test]
  fn test_ping_command() {
    let cmd = Command::Ping;

    let mut buf = Vec::<u8>::new();
    super::write_command(&cmd, &mut buf).expect("write ping command shouldn't fail");

    assert_eq!("*1\r\n$4\r\nPING\r\n\r\n".to_owned().into_bytes(), buf);
  }
}
