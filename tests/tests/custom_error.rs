use proto_rw::error::ProtoRwError;

#[derive(Debug)]
enum UserMadeError {
    Foo,
}

impl std::error::Error for UserMadeError {}

impl std::fmt::Display for UserMadeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Foo")
    }
}

fn do_something() -> Result<(), ProtoRwError> {
    Err(ProtoRwError::Error(UserMadeError::Foo.to_string()))
}

#[test]
fn custom_error() {
    if let Err(e) = do_something() {
        assert_eq!(e.to_string(), "Foo");
    }
}
