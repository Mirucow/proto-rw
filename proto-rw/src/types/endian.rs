use crate::{error::ProtoRwError, PRead, PWrite, ProtoRw};

pub struct LE<T>(pub T);

pub struct BE<T>(pub T);

impl ProtoRw for u8 {
    fn read<R: PRead>(buf: &mut R) -> Result<Self, ProtoRwError> {
        let mut data = [0; 1];
        buf.read_exact(&mut data)?;
        Ok(data[0])
    }

    fn write<W: PWrite>(&self, buf: &mut W) -> Result<(), ProtoRwError> {
        buf.write_all(&[*self])?;
        Ok(())
    }
}

impl ProtoRw for i8 {
    fn read<R: PRead>(buf: &mut R) -> Result<Self, ProtoRwError> {
        let mut data = [0; 1];
        buf.read_exact(&mut data)?;
        Ok(data[0] as i8)
    }

    fn write<W: PWrite>(&self, buf: &mut W) -> Result<(), ProtoRwError> {
        buf.write_all(&[*self as u8])?;
        Ok(())
    }
}

macro_rules! impl_endian {
    ($ty:ty) => {
        impl ProtoRw for LE<$ty> {
            fn read<R: PRead>(buf: &mut R) -> Result<Self, ProtoRwError> {
                let mut data = [0; std::mem::size_of::<$ty>()];
                buf.read_exact(&mut data)?;
                Ok(LE(<$ty>::from_le_bytes(data)))
            }

            fn write<W: PWrite>(&self, buf: &mut W) -> Result<(), ProtoRwError> {
                buf.write_all(&self.0.to_le_bytes())?;
                Ok(())
            }
        }

        impl ProtoRw for BE<$ty> {
            fn read<R: PRead>(buf: &mut R) -> Result<Self, ProtoRwError> {
                let mut data = [0; std::mem::size_of::<$ty>()];
                buf.read_exact(&mut data)?;
                Ok(BE(<$ty>::from_be_bytes(data)))
            }

            fn write<W: PWrite>(&self, buf: &mut W) -> Result<(), ProtoRwError> {
                buf.write_all(&self.0.to_be_bytes())?;
                Ok(())
            }
        }

        impl From<LE<$ty>> for $ty {
            fn from(data: LE<$ty>) -> Self {
                data.0
            }
        }

        impl From<BE<$ty>> for $ty {
            fn from(data: BE<$ty>) -> Self {
                data.0
            }
        }

        impl From<$ty> for LE<$ty> {
            fn from(data: $ty) -> Self {
                LE(data)
            }
        }

        impl From<$ty> for BE<$ty> {
            fn from(data: $ty) -> Self {
                BE(data)
            }
        }
    };
}

impl_endian!(u16);
impl_endian!(u32);
impl_endian!(u64);
impl_endian!(u128);
impl_endian!(i16);
impl_endian!(i32);
impl_endian!(i64);
impl_endian!(i128);
impl_endian!(f32);
impl_endian!(f64);
