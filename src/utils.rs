pub enum SupportedFileTypes {
    Jpeg,
    Png,
    Gif,
    WebP,
    Unkn,
}

pub fn get_filetype(file_path: &str) -> SupportedFileTypes {
    match infer::get_from_path(file_path) {
        Ok(v) => match v {
            None => SupportedFileTypes::Unkn,
            Some(ft) => match ft.mime_type() {
                "image/jpeg" => SupportedFileTypes::Jpeg,
                "image/png" => SupportedFileTypes::Png,
                "image/gif" => SupportedFileTypes::Gif,
                "image/webp" => SupportedFileTypes::WebP,
                _ => SupportedFileTypes::Unkn,
            },
        },
        Err(_) => SupportedFileTypes::Unkn,
    }
}
