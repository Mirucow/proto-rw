use crate::{error::ProtoRwError, PRead, PWrite, ProtoRw};

pub struct Var<T>(pub T);

macro_rules! read_varuint {
    ($buf:ident, $ty:ty) => {{
        let mut value = 0;
        let mut shift = 0;
        loop {
            let byte = $buf.read_proto::<u8>()?;
            value |= ((byte & 0x7F) as $ty) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
        }
        value
    }};
}

macro_rules! write_varuint {
    ($buf:ident, $value:expr) => {{
        let mut value = $value;
        loop {
            let byte = (value & 0x7F) as u8;
            value >>= 7;
            if value == 0 {
                $buf.write_proto(&byte)?;
                break;
            }
            $buf.write_proto(&(byte | 0x80))?;
        }
    }};
}

macro_rules! read_varint {
    ($buf:ident, $ty:ty) => {{
        let value = read_varuint!($buf, $ty);
        ((value >> 1) as $ty) ^ (-((value & 1) as $ty))
    }};
}

macro_rules! write_varint {
    ($buf:ident, $ty:ty, $value:expr) => {{
        let value = (($value << 1) as $ty)
            .wrapping_sub(($value >> (std::mem::size_of::<$ty>() * 8 - 1)) as $ty);
        write_varuint!($buf, value);
    }};
}

macro_rules! impl_varuint {
    ($ty:ty) => {
        impl ProtoRw for Var<$ty> {
            fn read<R: std::io::Read>(buf: &mut R) -> Result<Self, ProtoRwError> {
                Ok(Var(read_varuint!(buf, $ty)))
            }

            fn write<W: std::io::Write>(&self, buf: &mut W) -> Result<(), ProtoRwError> {
                write_varuint!(buf, self.0);
                Ok(())
            }
        }

        impl From<Var<$ty>> for $ty {
            fn from(value: Var<$ty>) -> Self {
                value.0
            }
        }

        impl From<$ty> for Var<$ty> {
            fn from(value: $ty) -> Self {
                Var(value)
            }
        }
    };
}

macro_rules! impl_varint {
    ($ty:ty) => {
        impl ProtoRw for Var<$ty> {
            fn read<R: std::io::Read>(buf: &mut R) -> Result<Self, ProtoRwError> {
                Ok(Var(read_varint!(buf, $ty)))
            }

            fn write<W: std::io::Write>(&self, buf: &mut W) -> Result<(), ProtoRwError> {
                write_varint!(buf, $ty, self.0);
                Ok(())
            }
        }

        impl From<Var<$ty>> for $ty {
            fn from(value: Var<$ty>) -> Self {
                value.0
            }
        }

        impl From<$ty> for Var<$ty> {
            fn from(value: $ty) -> Self {
                Var(value)
            }
        }
    };
}

impl_varuint!(u16);
impl_varuint!(u32);
impl_varuint!(u64);
impl_varuint!(u128);
impl_varint!(i16);
impl_varint!(i32);
impl_varint!(i64);
impl_varint!(i128);
