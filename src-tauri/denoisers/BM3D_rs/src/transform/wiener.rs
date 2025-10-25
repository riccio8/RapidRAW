//! first phase BM3D, signal estimation + refined filtering.
//!Parameters: sigma
//!
//!

//
// ### **Biorthogonal Wavelets**
// - **`Transform::from_wavelet(Wavelet::bior)`** - Creates a transform from a biorthogonal wavelet
// - **`Wavelet::new_biorthogonal(1, 3)`** - bior1.3, bior2.2, bior3.1, etc.
//
// ### **Forward 2D Transform**
// - **`transform_2d(&mut data, width, height, &transform, decomp_lvls)`**
// - **`forward(&mut data, width, height, levels)`** - Simplified version
//
// ### **Inverse 2D Transform**
// - **`inverse_2d(&mut data, width, height, &transform, decomp_lvls)`**
// - **`inverse(&mut data, width, height, levels)`** - Simplified version
//
// ### **Wavelet Utilities**
// - **`get_biorthogonal_wavelet()`** - Predefined biorthogonal wavelets
// - **`decomposition_steps(width, height, levels)`** - Computes decomposition dimensions
//
// **Returns**: `Result<(), TransformError>` - modifies buffer in-place
//
// ---
//
//
// ### **Constructing Biorthogonal Wavelets**
// - **`Wavelet::biorthogonal(vanishing_moments_decomposition, vanishing_moments_reconstruction)`**
// - Example: `bior2.2` → `Wavelet::biorthogonal(2, 2)`
//
// - **`Wavelet::bior1_3()`** - bior1.3
// - **`Wavelet::bior2_2()`** - bior2.2
// - **`Wavelet::bior3_1()`** - bior3.1
// - **`Wavelet::bior3_3()`** - bior3.3
// - **`Wavelet::bior3_5()`** - bior3.5
// - **`Wavelet::bior4_4()`** - bior4.4
//
// **Parameters**: `decomp_lvls` = number of decomposition levels (3-5)
//
