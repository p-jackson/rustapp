extern crate encoding;

use winapi::winnt::LPCWSTR;

struct U16ByteWriter {
  v: Vec<u16>,
  first_byte_next: bool
}

impl U16ByteWriter {
  fn new() -> U16ByteWriter {
    U16ByteWriter { v: Vec::new(), first_byte_next: true }
  }
}

impl encoding::types::ByteWriter for U16ByteWriter {

  fn write_byte(&mut self, b: u8) {
    if self.first_byte_next {
      self.v.push(b as u16);
    }
    else {
      let l = self.v.len();
      self.v[l - 1] = self.v[l - 1] | ((b as u16) << 8);
    }
    self.first_byte_next = !self.first_byte_next;
  }

  fn write_bytes(&mut self, bs: &[u8]) {
    for b in bs {
      self.write_byte(*b);
    }
  }

  fn writer_hint(&mut self, expectedlen: usize) {
    let len = self.v.len();
    self.v.reserve(expectedlen - len);
  }

}

pub fn to_wstring(s : &str) -> Vec<u16> {
  use encoding::all::UTF_16LE;
  use encoding::{Encoding, EncoderTrap};

  let mut writer = U16ByteWriter::new();
  UTF_16LE.encode_to(s, EncoderTrap::Strict, &mut writer).unwrap();
  let mut v = writer.v;
  v.push(0);
  v
}

pub enum ResourceName {
  Str(String),
  ID(LPCWSTR)
}

impl ResourceName {
  #[allow(dead_code)]
  pub fn from_str<S>(s: S) -> ResourceName where S: Into<String> {
    ResourceName::Str(s.into())
  }
  pub fn from_id(id: LPCWSTR) -> ResourceName {
    ResourceName::ID(id)
  }
  pub fn as_lpcwstr(&self) -> LPCWSTR {
    match *self {
      ResourceName::Str(ref s) => to_wstring(s.as_ref()).as_ptr() as LPCWSTR,
      ResourceName::ID(id) => id
    }
  }
}
