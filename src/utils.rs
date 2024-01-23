use core::fmt;
use infer::Type;

pub enum SupportedFileTypes {
    Jpeg,
    Png,
    Gif,
    WebP,
    Unkn,
}

pub fn get_filetype_from_path(file_path: &str) -> SupportedFileTypes {
    match infer::get_from_path(file_path) {
        Ok(v) => match v {
            None => SupportedFileTypes::Unkn,
            Some(ft) => match_supported_filetypes(ft),
        },
        Err(_) => SupportedFileTypes::Unkn,
    }
}

pub fn get_filetype_from_memory(buf: &[u8]) -> SupportedFileTypes {
    match infer::get(buf) {
        None => SupportedFileTypes::Unkn,
        Some(ft) => match_supported_filetypes(ft),
    }
}

fn match_supported_filetypes(ft: Type) -> SupportedFileTypes {
    match ft.mime_type() {
        "image/jpeg" => SupportedFileTypes::Jpeg,
        "image/png" => SupportedFileTypes::Png,
        "image/gif" => SupportedFileTypes::Gif,
        "image/webp" => SupportedFileTypes::WebP,
        _ => SupportedFileTypes::Unkn,
    }
}

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
