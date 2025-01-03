
pub enum CompressionError {
    IoError(std::io::Error),
    InvalidFormat,
    UnsupportedAlgorithm
}

pub fn map_to_error(err: std::io::Error) -> CompressionError {
    todo!()
}
