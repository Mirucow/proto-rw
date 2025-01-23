use bytes::BytesMut;
use proto_rw::{
    types::{BE, LE},
    ProtoRw,
};

#[test]
fn endian_rw() {
    let mut buf = BytesMut::new();

    BE::<i16>(-123).write_proto(&mut buf).unwrap();
    LE::<u32>(12345).write_proto(&mut buf).unwrap();
    BE::<i64>(-1234567).write_proto(&mut buf).unwrap();
    LE::<u128>(123456789).write_proto(&mut buf).unwrap();

    let mut buf = buf.freeze();

    assert_eq!(BE::<i16>::read_proto(&mut buf).unwrap().0, -123);
    assert_eq!(LE::<u32>::read_proto(&mut buf).unwrap().0, 12345);
    assert_eq!(BE::<i64>::read_proto(&mut buf).unwrap().0, -1234567);
    assert_eq!(LE::<u128>::read_proto(&mut buf).unwrap().0, 123456789);
}
