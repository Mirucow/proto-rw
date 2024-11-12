use crate::{error::ProtoRwError, PRead, PWrite, ProtoRw};

impl ProtoRw for bool {
    fn read<R: PRead>(buf: &mut R) -> Result<Self, ProtoRwError> {
        let value = buf.read_proto::<u8>()?;
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(crate::error::ProtoRwError::Error(format!(
                "Get {} while reading bool. Expected 0 or 1",
                value
            ))),
        }
    }

    fn write<W: PWrite>(&self, buf: &mut W) -> Result<(), ProtoRwError> {
        let value = if *self { 1 } else { 0 };
        buf.write_proto::<u8>(&value)
    }
}
