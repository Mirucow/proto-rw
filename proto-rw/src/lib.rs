use bytes::{Bytes, BytesMut};
use error::ProtoRwError;

pub mod error;
pub mod types;

pub extern crate macros;

pub trait ProtoRw: Sized {
    fn read_proto(buf: &mut Bytes) -> Result<Self, ProtoRwError>;
    fn write_proto(&self, buf: &mut BytesMut) -> Result<(), ProtoRwError>;
}
