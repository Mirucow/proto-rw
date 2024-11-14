pub enum ProtoRwError {
    IoError(std::io::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    Error(String),
}

impl ProtoRwError {
    pub fn to_string(&self) -> String {
        match self {
            ProtoRwError::IoError(e) => format!("Io error: {}", e),
            ProtoRwError::FromUtf8Error(e) => format!("FromUtf8 error: {}", e),
            ProtoRwError::Error(e) => format!("{}", e),
        }
    }
}

impl std::error::Error for ProtoRwError {}

impl From<std::io::Error> for ProtoRwError {
    fn from(e: std::io::Error) -> Self {
        ProtoRwError::IoError(e)
    }
}

impl From<std::string::FromUtf8Error> for ProtoRwError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        ProtoRwError::FromUtf8Error(e)
    }
}

impl std::fmt::Display for ProtoRwError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::fmt::Debug for ProtoRwError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
