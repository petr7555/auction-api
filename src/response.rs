use serde::Serialize;
use std::error;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    pub fn from(message: String) -> ErrorResponse {
        ErrorResponse { error: message }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Response Error: {}", self.error)
    }
}

impl error::Error for ErrorResponse {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Serialize, Debug)]
pub struct SuccessResponse {
    pub success: bool,
}
