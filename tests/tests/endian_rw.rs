use proto_rw::{types::{BE, LE}, ProtoRw};

#[test]
fn endian_rw() {
    let mut buf = vec![];

    BE::<i16>(-123).write_proto(&mut buf).unwrap();
    LE::<u32>(12345).write_proto(&mut buf).unwrap();
    BE::<i64>(-1234567).write_proto(&mut buf).unwrap();
    LE::<u128>(123456789).write_proto(&mut buf).unwrap();

    let mut cursor = std::io::Cursor::new(buf.as_mut_slice());

    assert_eq!(BE::<i16>::read_proto(&mut cursor).unwrap().0, -123);
    assert_eq!(LE::<u32>::read_proto(&mut cursor).unwrap().0, 12345);
    assert_eq!(BE::<i64>::read_proto(&mut cursor).unwrap().0, -1234567);
    assert_eq!(LE::<u128>::read_proto(&mut cursor).unwrap().0, 123456789);
}
