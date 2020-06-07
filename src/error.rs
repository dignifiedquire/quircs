use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Invalid grid size")]
    InvalidGridSize,
    #[error("Invalid version")]
    InvalidVersion,
    #[error("Format data ECC failure")]
    DataEcc,
    #[error("ECC failure")]
    FormatEcc,
    #[error("Unknown data type")]
    UnkownDataType,
    #[error("Data overflow")]
    DataOverflow,
    #[error("Data underflow")]
    DataUnderflow,
}
