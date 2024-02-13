use core::fmt;

pub type Result<T> = std::result::Result<T, CaesiumError>;

#[derive(Debug, Clone)]
pub struct CaesiumError {
    pub message: String,
    pub code: u32,
}

impl fmt::Display for CaesiumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{}]", self.message, self.code)
    }
}
