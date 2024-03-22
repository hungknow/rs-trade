pub type Result<T> = std::result::Result<T, TaError>;

#[derive(Debug)]
pub enum TaError {
    InvalidParameter,
    CsvError(csv::Error),
    CsvMissingColumn(String),
    HkTradingError(hktrading_client::types::HkError),
    UnknownError(String),
}

impl From<csv::Error> for TaError {
    fn from(e: csv::Error) -> Self {
        TaError::CsvError(e)
    }
}
