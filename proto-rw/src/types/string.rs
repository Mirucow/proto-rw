use crate::{error::ProtoRwError, PRead, PWrite, ProtoRw};

use super::var::Var;

impl ProtoRw for String {
    fn read<R: PRead>(buf: &mut R) -> Result<Self, ProtoRwError> {
        let len: u32 = buf.read_proto::<Var<u32>>()?.into();
        let mut data = vec![0; len as usize];
        buf.read_exact(&mut data)?;
        Ok(String::from_utf8(data)?)
    }

    fn write<W: PWrite>(&self, buf: &mut W) -> Result<(), ProtoRwError> {
        let data = self.as_bytes();
        let len = Var(data.len() as u32);
        buf.write_proto(&len)?;
        buf.write_all(data)?;
        Ok(())
    }
}
