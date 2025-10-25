//! bm3d_rs implementation

#![warn(non_snake_case)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::nursery)]
#![deny(clippy::todo)]

use image::DynamicImage;
use std::collections::HashMap;

/// wrapper for color operations
pub mod color;

/// wrapper for block operations
pub mod blocks;

/// wrapper for transform operations
pub mod transform;

/// wrapper for threshold operations
pub mod threshold;

/// wrapper for utility operations
pub mod utils;

/// wrapper for BM3D operations
pub mod bm3d;

/// public api for BM3D denoise operations
pub use bm3d::denoise;

/// public api for bm3d errors
pub mod error;


#[derive(Clone, Debug)]
/// BM3D image wrapper
pub struct Bm3dImage {
    /// image descriptor
    image: DynamicImage,
    /// image parameters for denoise
    params: Bm3dParams,
}


/// parameters for BM3D denoise operations
#[derive(Debug, Clone,  Default)]
pub struct Bm3dParams {
    /// all parameters for denoise
    pub params: HashMap<Parameters, ParamValue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// enum for parameters
pub enum Parameters {
    /// sigma value, variance of the noise (25)
    Sigma,
    /// lambda value for 2D (2.0)
    Lamb2D,
    /// lambda value for 3D (2.7)
    Lamb3D,
    /// kaiser window beta value (2 or 2.5)
    KaiserWindowBeta,
    /// step 1 threshold distance (2500)
    Step1ThresholdDist,
    /// step 1 max match (16)
    Step1MaxMatch,
    /// step 1 block size (8)
    Step1BlockSize,
    /// step 1 speedup factor, pixel jump for new reference block (3)
    Step1SpeedupFactor,
    /// step 1 window size (39)
    Step1WindowSize,
    /// step 2 threshold distance (400)
    Step2ThresholdDist,
    /// step 2 max match (32)
    Step2MaxMatch,
    /// step 2 block size (8)
    Step2BlockSize,
    /// step 2 speedup factor (3)
    Step2SpeedupFactor,
    /// step 2 window size (39)
    Step2WindowSize,
    /// luminance only
    LuminanceOnly,
    /// mix
    Mix,
    /// residual
    Residual,
}

#[derive(Debug, Clone, Copy)]
/// enum for parameter values
pub enum ParamValue {
    /// float value
    F64(f64),
    /// integer value
    I32(i32),
    /// boolean value
    Bool(bool),
}

impl Bm3dParams {
    /// constructor for Bm3dParams struct
    pub fn new() -> Self {
        use ParamValue::*;
        use Parameters::*;

        let mut params = HashMap::new();
        params.insert(Sigma, F64(25.0));
        params.insert(Lamb2D, F64(2.0));
        params.insert(Lamb3D, F64(2.7));
        params.insert(KaiserWindowBeta, F64(2.0));
        params.insert(LuminanceOnly, Bool(false));
        params.insert(Mix, F64(0.0));
        params.insert(Residual, Bool(false));
        params.insert(Step1ThresholdDist, I32(2500));
        params.insert(Step1MaxMatch, I32(16));
        params.insert(Step1BlockSize, I32(8));
        params.insert(Step1SpeedupFactor, I32(3));
        params.insert(Step1WindowSize, I32(39));
        params.insert(Step2ThresholdDist, I32(400));
        params.insert(Step2MaxMatch, I32(32));
        params.insert(Step2BlockSize, I32(8));
        params.insert(Step2SpeedupFactor, I32(3));
        params.insert(Step2WindowSize, I32(39));

        Self { params }
    }
    
    /// setter for Bm3dParams struct
    pub fn set(&mut self, key: Parameters, value: ParamValue) {
        self.params.insert(key, value);
    }
    
    /// getter for Bm3dParams struct
    pub fn get(&self, key: &Parameters) -> Option<&ParamValue> {
        self.params.get(key)
    }
}

/// struct Margin
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Margin {
    top_left: (i32, i32),
    bottom_right: (i32, i32),
}

impl Margin{
    ///constructor for new margin struct
    pub fn new(top_left: (i32, i32), bottom_right: (i32, i32)) -> Self {
        Self { top_left, bottom_right }
    }
    
    ///setter for Margin struct
    pub fn set(&mut self, top_left: (i32, i32), bottom_right: (i32, i32)) {
        self.top_left = top_left;
        self.bottom_right = bottom_right;
    }
    
    ///getter for Margin struct
    pub fn get(&self) -> ((i32, i32), (i32, i32)) {
        (self.top_left, self.bottom_right)
    }
}

/// struct Point 
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Point {
    x: i32,
    y: i32,
}
