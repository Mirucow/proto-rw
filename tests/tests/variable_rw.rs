use proto_rw::{types::Var, ProtoRw};

#[test]
fn variable_rw() {
    let mut buf = vec![];

    Var::<i16>(-123).write_proto(&mut buf).unwrap();
    Var::<u32>(12345).write_proto(&mut buf).unwrap();
    Var::<i64>(-1234567).write_proto(&mut buf).unwrap();
    Var::<u128>(123456789).write_proto(&mut buf).unwrap();

    let mut cursor = std::io::Cursor::new(buf.as_mut_slice());

    assert_eq!(Var::<i16>::read_proto(&mut cursor).unwrap().0, -123);
    assert_eq!(Var::<u32>::read_proto(&mut cursor).unwrap().0, 12345);
    assert_eq!(Var::<i64>::read_proto(&mut cursor).unwrap().0, -1234567);
    assert_eq!(Var::<u128>::read_proto(&mut cursor).unwrap().0, 123456789);
}
