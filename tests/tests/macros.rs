use proto_rw::{
    types::{Var, BE, LE},
    PRead, PWrite,
};
use macros::proto_rw;

#[derive(Debug, Clone, PartialEq)]
#[proto_rw]
struct ExampleStruct {
    a: u8,
    b: String,
    c: BE<i16>,
    d: LE<u32>,
    e: Var<u64>,
    f: ExampleEnum,
    #[length(Var<u32>)]
    g: Vec<u8>,
    #[length(BE<u16>)]
    h: Vec<ExampleEnum>,
    #[length(Var<u32>, Var<u16>)]
    i: Vec<Vec<u8>>,
    j: (LE<i32>, bool),
    #[length(Var<u32>)]
    k: (Vec<u8>, BE<u16>),
    m: (bool, (bool, String)),
    n: [u8; 4],
    #[convert(bool[0])]
    l: ExampleConvert,
    #[length(Var<u32>)]
    #[convert(bool[1], bool[2])]
    o: (String, ExampleConvert, Vec<ExampleConvert>),
}

#[derive(Debug, Clone, PartialEq)]
#[proto_rw(BE<i32>)]
enum ExampleEnum {
    A = 0,
    B = 1,
    C = 2,
}

#[derive(Clone)]
#[proto_rw]
struct ExampleConvert(pub bool);

impl From<ExampleConvert> for bool {
    fn from(data: ExampleConvert) -> Self {
        data.0
    }
}

impl From<bool> for ExampleConvert {
    fn from(data: bool) -> Self {
        ExampleConvert(data)
    }
}

#[test]
fn macros() {
    let example = ExampleStruct {
        a: 42,
        b: "hello".to_string(),
        c: -100,
        d: 1000,
        e: 123456,
        f: ExampleEnum::B,
        g: vec![1, 2, 3, 4],
        h: vec![ExampleEnum::A, ExampleEnum::B, ExampleEnum::C],
        i: vec![vec![1, 2, 3], vec![4, 5, 6]],
        j: (123, true),
        k: (vec![1, 2, 3], 1000),
        m: (true, (false, "world".to_string())),
        n: [1, 2, 3, 4],
        l: true,
        o: ("world".to_string(), false, vec![true, false]),
    };

    let mut buf = Vec::new();
    buf.write_proto(&example).unwrap();

    let mut cursor = std::io::Cursor::new(&buf);
    let read_example = cursor.read_proto::<ExampleStruct>().unwrap();

    assert_eq!(example, read_example);
}
