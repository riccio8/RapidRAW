# BM3D_rs
BM3D image processing algorithm implementation written in rust

## PARAMETERS

| Parameter | Description | Effect if Increased | Effect if Decreased |
|-----------|-------------|------------------|------------------|
| Sigma | Noise standard deviation (variance). Higher = more noise assumed. | Stronger denoising, may blur details. | Weaker denoising, more noise remains, but more details are preserved |
| Lamb2D | Lambda for 2D thresholding in step 1. | Stricter threshold, stronger denoise, may lose detail. | Softer threshold, preserves detail but less denoise. |
| Lamb3D | Lambda for 3D thresholding in step 2 (Wiener). | Stronger denoise, smoother image. | Weaker denoise, more noise remains. |
| KaiserWindowBeta | Beta value for Kaiser window in block transform (2–2.5 typical). | Sharper filtering, can reduce ringing. | Smoother filtering, may blur edges slightly. |
| Step1ThresholdDist | Distance threshold for grouping similar blocks in step 1. | Fewer blocks grouped, more selective, may keep details. | More blocks grouped, stronger denoise, may blur textures. |
| Step1MaxMatch | Max number of similar blocks to group in step 1. | More blocks grouped, stronger denoise, may blur textures. | Fewer blocks grouped, preserves detail, weaker denoise. |
| Step1BlockSize | Size of blocks in step 1 (e.g., 8×8). | Larger blocks, smoother denoise, may lose small details. | Smaller blocks, finer detail preserved, less denoise. |
| Step1SpeedupFactor | Pixel jump when searching new reference blocks. | Faster processing, may skip good matches, less accurate denoise. | Slower processing, more accurate block matching, better denoise. |
| Step1WindowSize | Search window size for similar blocks in step 1. | Larger window, finds more matches, stronger denoise, slower. | Smaller window, faster, may miss some matches, less denoise. |
| Step2ThresholdDist | Distance threshold for grouping in step 2 (Wiener). | Fewer blocks grouped, keeps details, weaker denoise. | More blocks grouped, stronger denoise, may blur textures. |
| Step2MaxMatch | Max similar blocks in step 2. | More blocks, stronger denoise, may blur. | Fewer blocks, preserves detail, weaker denoise. |
| Step2BlockSize | Block size in step 2. | Larger blocks, smoother denoise, may blur fine details. | Smaller blocks, preserves fine details, less denoise. |
| Step2SpeedupFactor | Pixel jump for new reference blocks in step 2. | Faster, may skip matches, weaker denoise. | Slower, more accurate matching, stronger denoise. |
| Step2WindowSize | Search window size in step 2. | Larger window, stronger denoise, slower. | Smaller window, weaker denoise, faster. |
| LuminanceOnly | Apply denoise only to luminance channel. | Only luminance is filtered, color preserved. | N/A – turning off will denoise all channels. |
| Mix | Mix between step1 and step2 results. | More from step2 = smoother, stronger denoise. | More from step1 = more texture/detail preserved. |
| Residual | Return residual (noise removed) instead of denoised image. | N/A – outputs noise. | N/A – outputs noise. |


### FULL IMPLEMNTATION DOCUMENTATION 
https://docs.google.com/viewerng/viewer?url=https://www.ipol.im/pub/art/2012/l-bm3d//article_lr.pdf
