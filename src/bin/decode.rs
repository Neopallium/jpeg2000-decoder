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
const MAX_DIM_LIMIT: u32 = 8192;          // bigger than this, we're not going to reduce
const JPEG_2000_COMPRESSION_FACTOR: f32 = 0.9;    // conservative estimate of how much JPEG 2000 reduces size
const BYTES_PER_PIXEL: usize = 4;             // assume RGBA, 8 bits
const MINIMUM_SIZE_TO_READ: usize = 4096;   // smaller than this and JPEG 2000 files break down.
/// Estimate amount of data to read for a desired resolution.
/// This should overestimate, so we read enough.
///
/// Returns (max bytes, discard level).
/// Discard level 0 is full size, 1 is 1/4 size, etc. 
pub fn estimate_read_size(image_size: (u32, u32), max_dim: u32) -> (usize, usize) {
    assert!(max_dim > 0);       // would cause divide by zero
    if max_dim > MAX_DIM_LIMIT {
        return (usize::MAX, 0)      // full size
    }
    let reduction_ratio = (image_size.0.max(image_size.1)) as usize / (max_dim as usize);
    let in_pixels = (image_size.0 as usize) * (image_size.1 as usize);
    let out_pixels = in_pixels / reduction_ratio;   // number of pixels desired in output
    //  Read this many bytes and decode.
    let max_bytes = (((out_pixels * BYTES_PER_PIXEL) as f32) * JPEG_2000_COMPRESSION_FACTOR) as usize;
    let max_bytes = max_bytes.max(MINIMUM_SIZE_TO_READ);
    //  Reduction ratio 1 -> discard level 0, 4->1, 16->2, etc. Round down.
    let discard_level = calc_discard_level(reduction_ratio);  // ***SCALE***
    (max_bytes, discard_level)
}

///  Reduction ratio 1 -> discard level 0, 4->1, 16->2, etc. Round up.
fn calc_discard_level(reduction_ratio: usize) -> usize {
    assert!(reduction_ratio > 0);
    for i in 0..16 {
        if 4_u32.pow(i) as usize >= reduction_ratio {
            return i.try_into().expect("calc discard level overflow")
        }
    }
    panic!("Argument to calc_discard_level is out of range.");
}

/// Estimate when we don't know what the image size is.
pub fn estimate_initial_read_size(max_dim: u32) -> usize {
    let square = |x| x*x;           // ought to be built in
    if max_dim > MAX_DIM_LIMIT {
        usize::MAX                   // no limit
    } else {
        ((square(max_dim as f32) * BYTES_PER_PIXEL as f32 * JPEG_2000_COMPRESSION_FACTOR) as usize).max(MINIMUM_SIZE_TO_READ)
    }
}

#[test]
/// Sanity check on estimator math
fn test_calc_discard_level() {
    assert_eq!(calc_discard_level(1), 0);
    assert_eq!(calc_discard_level(2), 1);
    assert_eq!(calc_discard_level(3), 1);
    assert_eq!(calc_discard_level(4), 1);
    assert_eq!(calc_discard_level(5), 2);
    assert_eq!(calc_discard_level(15), 2);
    assert_eq!(calc_discard_level(16), 2);
    assert_eq!(calc_discard_level(17), 3);
    assert_eq!(calc_discard_level(63), 3);
    assert_eq!(calc_discard_level(64), 3);
    assert_eq!(calc_discard_level(65), 4);
}
