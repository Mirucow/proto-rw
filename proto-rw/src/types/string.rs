use std::io::{Cursor, Read, Write};

use crate::{error::ProtoRwError, ProtoRw};

use super::var::Var;

impl ProtoRw for String {
    fn read_proto(buf: &mut Cursor<&mut [u8]>) -> Result<Self, ProtoRwError> {
        let len = Var::<u32>::read_proto(buf)?.0;
        let mut data = vec![0; len as usize];
        buf.read_exact(&mut data)?;
        Ok(String::from_utf8(data)?)
    }

    fn write_proto(&self, buf: &mut Vec<u8>) -> Result<(), ProtoRwError> {
        let data = self.as_bytes();
        let len = data.len() as u32;
        Var(len).write_proto(buf)?;
        buf.write_all(data)?;
        Ok(())
    }
}
