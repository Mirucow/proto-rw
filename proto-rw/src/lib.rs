use error::ProtoRwError;

pub mod error;
pub mod types;

pub extern crate macros;

pub trait ProtoRw: Sized {
    fn read<R: PRead>(buf: &mut R) -> Result<Self, ProtoRwError>;
    fn write<W: PWrite>(&self, buf: &mut W) -> Result<(), ProtoRwError>;
}

pub trait PRead: std::io::Read + Sized {
    fn read_proto<T: ProtoRw>(&mut self) -> Result<T, ProtoRwError> {
        Ok(T::read(self)?)
    }
}

impl<R: std::io::Read + Sized> PRead for R {}

pub trait PWrite: std::io::Write + Sized {
    fn write_proto<T: ProtoRw>(&mut self, ty: &T) -> Result<(), ProtoRwError> {
        ty.write(self)
    }
}

impl<W: std::io::Write + Sized> PWrite for W {}
