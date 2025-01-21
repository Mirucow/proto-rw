use std::io::Cursor;

use error::ProtoRwError;

pub mod error;
pub mod types;

pub extern crate macros;

pub trait ProtoRw: Sized {
    fn read_proto(buf: &mut Cursor<&mut [u8]>) -> Result<Self, ProtoRwError>;
    fn write_proto(&self, buf: &mut Vec<u8>) -> Result<(), ProtoRwError>;
}
