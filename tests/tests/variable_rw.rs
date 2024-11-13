use proto_rw::{types::Var, PRead, PWrite};

#[test]
fn variable_rw() {
    let mut buf = vec![];

    buf.write_proto(&Var::<u16>(123)).unwrap();
    // you can write the type directly (requires From trait)
    buf.write_proto_from::<u32, Var<u32>>(12345).unwrap();
    buf.write_proto(&Var::<u64>(1234567)).unwrap();
    buf.write_proto(&Var::<i16>(-123)).unwrap();
    buf.write_proto(&Var::<i32>(-12345)).unwrap();
    buf.write_proto(&Var::<i64>(-1234567)).unwrap();

    let mut cursor = std::io::Cursor::new(&buf);

    assert_eq!(u16::from(cursor.read_proto::<Var<u16>>().unwrap()), 123);
    // you can read the type directly (requires Into trait)
    assert_eq!(cursor.read_proto_into::<Var<u32>, u32>().unwrap(), 12345);
    assert_eq!(u64::from(cursor.read_proto::<Var<u64>>().unwrap()), 1234567);
    assert_eq!(i16::from(cursor.read_proto::<Var<i16>>().unwrap()), -123);
    assert_eq!(i32::from(cursor.read_proto::<Var<i32>>().unwrap()), -12345);
    assert_eq!(
        i64::from(cursor.read_proto::<Var<i64>>().unwrap()),
        -1234567
    );
}
