//! conversion functions from rgb to YCbCr color space, if luminance_only is true working only on Y channel,
//! otherwise working on all channels.
use crate::{Bm3dParams, Bm3dImage};
use crate::error::ImageProcessingError;
use image::{DynamicImage, ImageBuffer};
use zune_image::{
    image::Image, 
    codecs::bmp::zune_core::colorspace::ColorSpace, 
};


impl Bm3dImage {
    /// constructor for Bm3dImage struct
    pub fn new(
        image: DynamicImage,
        params: Bm3dParams,
    ) -> Self {
        Self {
            image,
            params,
        }
    }
    
    /// Convert a DynamicImage to YCbCr color space using Zune Image library.
    pub fn convert_dynamic_to_ycbcr(dynamic_img: &DynamicImage) -> Result<Image, ImageProcessingError> {
        let rgb_img = dynamic_img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        
        let mut zune_img = Image::from_u8(
            &rgb_img,
            width as usize, 
            height as usize,
            ColorSpace::RGB
        );

        zune_img.convert_color(ColorSpace::YCbCr).map_err(|_| ImageProcessingError::ColorConversionError)?;
        Ok(zune_img)
    }
    
    /// convert YCbCr to DynamicImage
    pub fn convert_ycbcr_to_dynamic(zune_img: Image) -> Result<DynamicImage, ImageProcessingError> {
        let mut img_clone = zune_img.clone();
        
        img_clone.convert_color(ColorSpace::RGB).map_err(|_| ImageProcessingError::ColorConversionError)?;
        
        let (width, height) = img_clone.dimensions();
        let data = img_clone.flatten_to_u8().remove(0); 
        
        Ok(DynamicImage::ImageRgb8(
            ImageBuffer::from_raw(width as u32, height as u32, data)
                .ok_or_else(|| ImageProcessingError::UnsupportedFormat("Invalid raw buffer".into()))?
        ))
    }
    
    /// Convert a Vec<u8> to Zune Image format using Zune Image library.
    pub fn vec_to_zune(
        data: Vec<u8>,
        width: usize, 
        height: usize,
        colorspace: ColorSpace
    ) -> Image {
        Image::from_u8(&data, width, height, colorspace)
    }
    
    /// Convert a DynamicImage to Zune Image format using Zune Image library.
    pub fn dynamic_to_zune(dynamic_img: &image::DynamicImage) -> Image {
        let rgb_img = dynamic_img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        
        Self::vec_to_zune(
            rgb_img.into_vec(),  // &[u8] default from rgb image
            width as usize,
            height as usize,
            ColorSpace::RGB
        )
    }
    // will implement conversion functions from rgb to YCbCr color space manually
}

