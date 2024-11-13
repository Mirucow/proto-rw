use proto_rw::{
    types::{BE, LE},
    PRead, PWrite,
};

#[test]
fn endian_rw() {
    let mut buf = vec![];

    buf.write_proto(&BE::<i16>(-123)).unwrap();
    // you can write the type directly (requires From trait)
    buf.write_proto_from::<u32, LE<u32>>(12345).unwrap();
    buf.write_proto(&BE::<i64>(-1234567)).unwrap();
    buf.write_proto(&LE::<u128>(123456789)).unwrap();

    let mut cursor = std::io::Cursor::new(&buf);

    assert_eq!(i16::from(cursor.read_proto::<BE<i16>>().unwrap()), -123);
    // you can read the type directly (requires Into trait)
    assert_eq!(cursor.read_proto_into::<LE<u32>, u32>().unwrap(), 12345);
    assert_eq!(i64::from(cursor.read_proto::<BE<i64>>().unwrap()), -1234567);
    assert_eq!(
        u128::from(cursor.read_proto::<LE<u128>>().unwrap()),
        123456789
    );
}
