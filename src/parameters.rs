use crate::parameters::TiffCompression::Deflate;

#[derive(Copy, Clone, PartialEq)]
pub enum ChromaSubsampling {
    CS444,
    CS422,
    CS420,
    CS411,
    Auto,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TiffCompression {
    Uncompressed = 0,
    Lzw = 1,
    Deflate = 2,
    Packbits = 3,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TiffDeflateLevel {
    Fast = 1,
    Balanced = 6,
    Best = 9,
}
#[derive(Copy, Clone)]
pub struct JpegParameters {
    pub quality: u32,
    pub chroma_subsampling: ChromaSubsampling,
    pub progressive: bool
}

#[derive(Copy, Clone)]
pub struct PngParameters {
    pub quality: u32,
    pub force_zopfli: bool,
    pub optimization_level: u8,
}

#[derive(Copy, Clone)]
pub struct GifParameters {
    pub quality: u32,
}

#[derive(Copy, Clone)]
pub struct WebPParameters {
    pub quality: u32,
}

#[derive(Copy, Clone)]
pub struct TiffParameters {
    pub algorithm: TiffCompression,
    pub deflate_level: TiffDeflateLevel,
}

#[derive(Copy, Clone)]
pub struct CSParameters {
    pub jpeg: JpegParameters,
    pub png: PngParameters,
    pub gif: GifParameters,
    pub webp: WebPParameters,
    pub tiff: TiffParameters,
    pub keep_metadata: bool,
    pub optimize: bool,
    pub width: u32,
    pub height: u32,
    pub output_size: u32,
}
impl Default for CSParameters {
    fn default() -> Self {
        Self::new()
    }
}

impl CSParameters {
    pub fn new() -> CSParameters {
        initialize_parameters()
    }
}

fn initialize_parameters() -> CSParameters {
    let jpeg = JpegParameters {
        quality: 80,
        chroma_subsampling: ChromaSubsampling::Auto,
        progressive: true
    };
    let png = PngParameters {
        quality: 80,
        force_zopfli: false,
        optimization_level: 3,
    };
    let gif = GifParameters { quality: 80 };
    let webp = WebPParameters { quality: 80 };
    let tiff = TiffParameters {
        algorithm: Deflate,
        deflate_level: TiffDeflateLevel::Balanced,
    };

    CSParameters {
        jpeg,
        png,
        gif,
        webp,
        tiff,
        keep_metadata: false,
        optimize: false,
        width: 0,
        height: 0,
        output_size: 0,
    }
}