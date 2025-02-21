use crate::parameters::TiffCompression::Deflate;

/// Enum representing different chroma subsampling options for JPEG compression.
///
/// - `CS444`: 4:4:4 chroma subsampling
/// - `CS422`: 4:2:2 chroma subsampling
/// - `CS420`: 4:2:0 chroma subsampling
/// - `CS411`: 4:1:1 chroma subsampling
/// - `Auto`: Automatic chroma subsampling
#[derive(Copy, Clone, PartialEq)]
pub enum ChromaSubsampling {
    CS444,
    CS422,
    CS420,
    CS411,
    Auto,
}

/// Enum representing different compression algorithms for TIFF images.
///
/// - `Uncompressed`: No compression
/// - `Lzw`: LZW compression
/// - `Deflate`: Deflate compression
/// - `Packbits`: PackBits compression
#[derive(Copy, Clone, PartialEq)]
pub enum TiffCompression {
    Uncompressed = 0,
    Lzw = 1,
    Deflate = 2,
    Packbits = 3,
}

/// Enum representing different deflate levels for TIFF compression.
///
/// - `Fast`: Fast compression
/// - `Balanced`: Balanced compression
/// - `Best`: Best compression
#[derive(Copy, Clone, PartialEq)]
pub enum TiffDeflateLevel {
    Fast = 1,
    Balanced = 6,
    Best = 9,
}

/// Struct representing parameters for JPEG compression.
///
/// Fields:
/// - `quality`: Quality of the JPEG image (0-100)
/// - `chroma_subsampling`: Chroma subsampling option
/// - `progressive`: Whether to use progressive JPEG
#[derive(Copy, Clone)]
pub struct JpegParameters {
    pub quality: u32,
    pub chroma_subsampling: ChromaSubsampling,
    pub progressive: bool,
}

/// Struct representing parameters for PNG compression.
///
/// Fields:
/// - `quality`: Quality of the PNG image (0-100)
/// - `force_zopfli`: Whether to force the use of Zopfli compression (can be very slow)
/// - `optimization_level`: Optimization level for PNG compression (0-6)
#[derive(Copy, Clone)]
pub struct PngParameters {
    pub quality: u32,
    pub force_zopfli: bool,
    pub optimization_level: u8,
}

/// Struct representing parameters for GIF compression.
///
/// Fields:
/// - `quality`: Quality of the GIF image (0-100)
#[derive(Copy, Clone)]
pub struct GifParameters {
    pub quality: u32,
}

/// Struct representing parameters for WebP compression.
///
/// Fields:
/// - `quality`: Quality of the WebP image (0-100)
#[derive(Copy, Clone)]
pub struct WebPParameters {
    pub quality: u32,
}

/// Struct representing parameters for TIFF compression.
///
/// Fields:
/// - `algorithm`: Compression algorithm for TIFF
/// - `deflate_level`: Deflate level for TIFF compression
#[derive(Copy, Clone)]
pub struct TiffParameters {
    pub algorithm: TiffCompression,
    pub deflate_level: TiffDeflateLevel,
}

/// Struct representing overall compression parameters.
///
/// Fields:
/// - `jpeg`: JPEG compression parameters
/// - `png`: PNG compression parameters
/// - `gif`: GIF compression parameters
/// - `webp`: WebP compression parameters
/// - `tiff`: TIFF compression parameters
/// - `keep_metadata`: Whether to keep metadata in the compressed image
/// - `optimize`: Whether to use lossless compression
/// - `width`: Width of the output image
/// - `height`: Height of the output image
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
        progressive: true,
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
    }
}
