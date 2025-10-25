/// wrapper function for BM3D denoising algorithmx

use crate::error::ImageProcessingError;
use crate::{Margin, Point, Bm3dImage, Bm3dParams};

use zune_image::{
    image::Image,
};
use signal_transforms::dct::Dct2D;
use nalgebra::base::{
    OMatrix,
    dimension::Dyn,
};

use std::path::Path;


/// main denoise function accepting parameters
pub fn denoise(image_path: &Path, output_path: &Path, sigma: f64)  {
    todo!("Implement BM3D denoising algorithm")
}

//let ref_point = (
//(spdup_factor * i).min(basic_img_height - block_size - 1),
//(spdup_factor * j).min(basic_img_width - block_size - 1),
//);


///  Find the search window whose center is reference block in *Img*
/// Note that the center of SearchWindow is not always the reference block because of the border 
pub fn search_window(img: &Image, 
    ref_point: (usize, usize),
    block_size: usize,
    window_size: usize) -> Result<Margin, ImageProcessingError>{
            
    if block_size >= window_size {
            return Err(ImageProcessingError::InvalidParameter(
                "Invalid Image size, block size must be smaller than window size"
            ));
        }

    let mut left = f64::max(0.0, (ref_point.0 as f64 + (block_size as f64 - window_size as f64) / 2.0)); // left top x
    let mut top  = f64::max(0.0, (ref_point.1 as f64 + (block_size as f64 - window_size as f64) / 2.0)); // left top y
    let mut right = left + window_size as f64; // right bottom x
    let mut bottom = top + window_size as f64; // right bottom y
    
    if right >= img.dimensions().0 as f64{
        right = (img.dimensions().0 - 1) as f64;
        left = right - window_size as f64;
    }
    if bottom >= img.dimensions().1 as f64{
        bottom = (img.dimensions().1 - 1) as f64;
        top = bottom - window_size as f64; 
    }

    Ok(Margin {
        top_left: (left as i32, top as i32),
        bottom_right: (right as i32, bottom as i32),
    })
}

