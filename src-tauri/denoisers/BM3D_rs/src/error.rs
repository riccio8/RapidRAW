use std::fmt;

/// Central error handling for image processing operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageProcessingError {
    /// Error in color conversion
    ColorConversionError,
    /// Error in discrete cosine transform (DCT)
    DctError,
    /// Error in wavelet transform
    WaveletError,
    /// Invalid parameter, e.g. window size, threshold
    InvalidParameter(&'static str),
    /// Access out of bounds, e.g. pixel index, patch index
    OutOfBounds(&'static str),
    /// Unsupported image format
    UnsupportedFormat(&'static str),
    /// Other error
    Other(&'static str),
}

impl fmt::Display for ImageProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ColorConversionError => write!(f, "Error in color conversion"),
            Self::DctError => write!(f, "Error in discrete cosine transform (DCT)"),
            Self::WaveletError => write!(f, "Error in wavelet transform"),
            Self::InvalidParameter(param) => write!(f, "Invalid parameter: {}", param),
            Self::OutOfBounds(context) => write!(f, "Access out of bounds: {}", context),
            Self::UnsupportedFormat(fmt) => write!(f, "Unsupported image format: {}", fmt),
            Self::Other(msg) => write!(f, "Generic error: {}", msg),
        }
    }
}

/// Implement std::error::Error for compatibility with other Rust APIs
impl std::error::Error for ImageProcessingError {}

/// Common conversions
impl From<&'static str> for ImageProcessingError {
    fn from(s: &'static str) -> Self {
        ImageProcessingError::Other(s)
    }
}

impl From<ImageProcessingError> for &'static str {
    fn from(e: ImageProcessingError) -> Self {
        match e {
            ImageProcessingError::Other(msg) => msg,
            _ => "Generic error",
        }
    }
}

impl TryFrom<u32> for ImageProcessingError {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ImageProcessingError::ColorConversionError),
            1 => Ok(ImageProcessingError::DctError),
            2 => Ok(ImageProcessingError::WaveletError),
            _ => Err("Code error not valid"),
        }
    }
}

impl From<ImageProcessingError> for u32 {
    fn from(err: ImageProcessingError) -> Self {
        match err {
            ImageProcessingError::ColorConversionError => 0,
            ImageProcessingError::DctError => 1,
            ImageProcessingError::WaveletError => 2,
            _ => 999,
        }
    }
}
