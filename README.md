# ProtoRw

> [!WARNING]
> This library is made for personal use and is not recommended for production use.

ProtoRw is a library for reading and writing protocol buffers in Rust. It is designed to be easy to use and efficient.

Just add the `#[proto_rw]` attribute to your struct and ProtoRw will generate the code to read and write it.

## Features

- Automatic read/write implementation for structs and enums
- Support automatic conversion between types
- Support Big Endian, Little Endian and Variable Length integers
- Support for nested structs and enums
- Define custom read/write functions for your types

## Example


### Basic

```rust
use bytes::BytesMut;
use proto_rw::types::{Var, BE, LE};

// of course you can use the derive attribute
#[derive(Debug, Clone)]
#[proto_rw]
struct ExampleStruct {
    a: u8,
    b: String,
    c: BE<i16>,
    d: LE<u32>,
    e: Var<u64>,
    f: (u8, String),
}

fn main() {
    let example = ExampleStruct {
        a: 42,
        b: "hello".to_string(),
        c: -100,
        d: 1000,
        e: 123456,
        f: (10, "world".to_string()),
    };

    let mut buf = BytesMut::new();
    example.write_proto(&mut buf).unwrap();

    let mut buf = buf.freeze();
    let example2 = ExampleStruct::read_proto(&mut buf).unwrap();

    assert_eq!(example, example2);
}
```

### Define Vec length

```rust
#[proto_rw]
struct ExampleStruct {
    #[length(u8)]
    a: Vec<String>,
    #[length(u8, Var<u32>)]
    b: Vec<Vec<BE<i16>>>,
    #[length(Var<u16>, Var<u32>, Var<u64>, Var<u128>)]
    c: (Vec<u8>, Vec<u8>, Vec<Vec<u8>>),
}
```

## Contributing

Contributions are welcome! Feel free to submit a pull request.

## License

ProtoRw is licensed under the MIT license.