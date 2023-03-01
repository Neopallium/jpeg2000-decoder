//! # decode.rs  -- Decoder program for JPEG 2000 files.
//!
//  Animats
//  February, 2023
//

////use url::Url;
////use std::path::Path;
use anyhow::{Error};
use jpeg2k::*;
use image::{DynamicImage};
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use image::GenericImageView;

/// Maximum image dimension allowed
const MAX_DIM_ALLOWED: u32 = 8192;          // bigger than this, we're not going to reduce
const JPEG_2000_REDUCTION_FACTOR: f32 = 0.9;    // conservative estimate of how much JPEG 2000 reduces size
const BYTES_PER_PIXEL: usize = 4;             // assume RGBA, 8 bits
const MINIMUM_SIZE_TO_READ: usize = 4096;   // smaller than this and JPEG 2000 files break down.
/*
/// Estimate amount of data to read for a desired resolution.
/// This should overestimate, so we read enough.
pub fn estimate_read_size(image_size: Option<(u32, u32)>, max_dim: u32) -> usize {
    if max_dim > MAX_DIM_ALLOWED {
        return usize::MAX                   // no limit
    }
    let square = |x| x*x;           // ought to be built in
    let image_pixels_est = 
        if Some(size) = image_size { 
            (image_size.0 as f32) * (imagesize.1 as f32) / square(max_dim as f32) 
        } else {         
            square(max_dim as f32)  // guess
        };
        //  ***WRONG***  
}
*/

/// Estimate when we don't know what the image size is.
pub fn estimate_initial_read_size(max_dim: u32) -> usize {
    let square = |x| x*x;           // ought to be built in
    if max_dim > MAX_DIM_ALLOWED {
        usize::MAX                   // no limit
    } else {
        ((square(max_dim as f32) * BYTES_PER_PIXEL as f32 * JPEG_2000_REDUCTION_FACTOR) as usize).max(MINIMUM_SIZE_TO_READ)
    }
}



