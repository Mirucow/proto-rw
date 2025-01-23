use bytes::{Bytes, BytesMut};

use crate::{error::ProtoRwError, ProtoRw};

impl ProtoRw for bool {
    fn read_proto(buf: &mut Bytes) -> Result<Self, ProtoRwError> {
        let value = u8::read_proto(buf)?;
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(crate::error::ProtoRwError::Error(format!(
                "Get {} while reading bool. Expected 0 or 1",
                value
            ))),
        }
    }

    fn write_proto(&self, buf: &mut BytesMut) -> Result<(), ProtoRwError> {
        let value = if *self { 1 } else { 0 };
        u8::write_proto(&value, buf)?;
        Ok(())
    }
}
