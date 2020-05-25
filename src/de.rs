use crate::error::{Error, Result};
use serde::de::{self, IntoDeserializer, Visitor};
use serde::Deserialize;
use std::io::BufRead;

#[derive(Debug, Clone, PartialEq)]
enum MsgType {
  Command,
  Reply(Option<String>), // RESP error
}

struct Deserializer<'de, R: BufRead> {
  msg_type: MsgType,
  reader: &'de mut R,
}

/// T needs to be of Enum type
pub fn read_command<'a, R, T>(reader: &'a mut R) -> Result<T>
where
  T: Deserialize<'a>,
  R: BufRead,
{
  let mut deserializer = Deserializer {
    msg_type: MsgType::Command,
    reader,
  };
  let t = T::deserialize(&mut deserializer)?;
  Ok(t)
}

/// The inner std::result::Result wrapper would contain successfully read RESP Errors
/// The outer Result from this lib will capture deserialization failure if any
pub fn read_reply<'a, R, T>(reader: &'a mut R) -> Result<std::result::Result<T, String>>
where
  T: Deserialize<'a>,
  R: BufRead,
{
  let mut deserializer = Deserializer {
    msg_type: MsgType::Reply(None),
    reader,
  };
  let t = T::deserialize(&mut deserializer)?;
  // handles RESP error type
  match deserializer.msg_type {
    MsgType::Reply(None) => Ok(Ok(t)),
    MsgType::Reply(Some(err)) => Ok(Err(err)),
    MsgType::Command => unimplemented!(),
  }
}

// parsing utility
impl<'de, R: BufRead> Deserializer<'de, R> {
  fn read_byte(&mut self) -> Result<u8> {
    let mut buf = [0; 1];
    self.reader.read_exact(&mut buf)?;
    Ok(buf[0])
  }

  fn read_int(&mut self) -> Result<i64> {
    // TODO: stricter line ending, currently only cares about \n, need to check \r\n
    let mut buf = String::new();
    self.reader.read_line(&mut buf)?;
    Ok(buf.trim_end().parse()?)
  }

  fn read_string(&mut self) -> Result<String> {
    // TODO: stricter line ending
    // TODO: avoiding creating new String?
    let mut buf = String::new();
    self.reader.read_line(&mut buf)?;
    buf.pop();
    buf.pop();
    Ok(buf)
  }
}

impl<'de, 'a, R: BufRead> de::Deserializer<'de> for &'a mut Deserializer<'de, R> {
  type Error = Error;

  fn deserialize_any<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_bool<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_i8<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_i16<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_i32<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_i64<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_u8<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_u16<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_u32<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_u64<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_f32<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_f64<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_char<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_str<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    // read type
    match self.read_byte()? {
      b'+' => visitor.visit_string(self.read_string()?),
      b'-' => {
        let err = self.read_string()?;
        self.msg_type = MsgType::Reply(Some(err));
        visitor.visit_string("".to_owned())
      }
      _ => unimplemented!(),
    }
  }

  fn deserialize_bytes<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_option<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_unit<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_newtype_struct<V>(self, _: &'static str, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_seq<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_tuple_struct<V>(self, _: &'static str, _: usize, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_map<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_struct<V>(self, _: &'static str, _: &'static [&'static str], _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_enum<V>(self, _: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    if self.msg_type != MsgType::Command {
      // TODO: return error
      unimplemented!();
    }

    // read command size
    let typ = self.read_byte()?;
    if typ != b'*' {
      return Err(Error::InvalidTypeForCommand);
    }

    let cmd_size = self.read_int()?;
    match cmd_size {
      x if x <= 0 => Err(Error::InvalidSizeForCommandArray),
      1 => {
        // unit_variant
        let _ = self.read_int(); // ignoring length
        let cmd = self.read_string()?;
        for var in variants {
          if var.to_uppercase() == cmd.to_uppercase() {
            return visitor.visit_enum(var.into_deserializer());
          }
        }
        unimplemented!()
      }
      2 => {
        // newtype_variant
        unimplemented!()
      }
      _n => {
        // tuple_variant
        unimplemented!()
      }
    }
  }

  fn deserialize_identifier<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }
}

#[cfg(test)]
mod test {
  use serde::Deserialize;

  #[derive(Debug, PartialEq, Deserialize)]
  enum Command {
    Ping,
  }

  #[test]
  fn test_ping_command() {
    let mut buf = "*1\r\n$4\r\nPING\r\n\r\n".as_bytes();
    let cmd = super::read_command(&mut buf).expect("read ping command shouldn't fail");

    assert_eq!(Command::Ping, cmd);
  }

  #[test]
  fn test_pong_reply() {
    let mut buf = "+PONG\r\n".as_bytes();
    let reply = super::read_reply(&mut buf).expect("read PONG reply shouldn't fail");

    assert_eq!(Ok("PONG".to_owned()), reply);
  }

  #[test]
  fn test_error_reply() {
    let mut buf = "-FAIL\r\n".as_bytes();
    let reply: std::result::Result<String, String> = super::read_reply(&mut buf).expect("read Error shouldn't fail");

    assert_eq!(Err("FAIL".to_owned()), reply);
  }
}
