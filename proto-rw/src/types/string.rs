use bytes::{Buf, Bytes, BytesMut};

use crate::{error::ProtoRwError, ProtoRw};

use super::var::Var;

impl ProtoRw for String {
    fn read_proto(buf: &mut Bytes) -> Result<Self, ProtoRwError> {
        let len = Var::<u32>::read_proto(buf)?.0;

        if buf.remaining() < len as usize {
            return Err(ProtoRwError::UnexpectedEof);
        }
        
        let mut data = vec![0; len as usize];
        buf.copy_to_slice(&mut data);
        Ok(String::from_utf8(data)?)
    }

    fn write_proto(&self, buf: &mut BytesMut) -> Result<(), ProtoRwError> {
        let data = self.as_bytes();
        let len = data.len() as u32;
        Var(len).write_proto(buf)?;
        buf.extend_from_slice(data);
        Ok(())
    }
}
