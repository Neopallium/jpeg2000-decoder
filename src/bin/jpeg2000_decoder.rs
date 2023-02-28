//! # jpeg2000-decoder  -- Decoder program for JPEG 2000 files.
//!
//  Animats
//  April, 2021
//

////use url::Url;
////use std::path::Path;
use anyhow::{Error};
use jpeg2k::*;
use image::{DynamicImage};




/// Arguments to the program
#[derive(Clone, Debug, Default)]
struct ArgInfo {
    /// Source URL
    pub in_url: String,
    /// Destination file
    pub out_file: String,
    /// Maximum output image dimension, in pixels
    pub max_size: u32,
    /// If true, ignore above fields and read LLSD commands from input.
    pub llsd_mode: bool,
    /// Verbose mode. Goes to standard error if LLSD mode.
    pub verbose: bool,
    /// User agent for HTTP requests.
    pub user_agent: String
}

//
//  parseargs -- parse command line args
//
//  Sets options, returns file to process
//
fn parseargs() -> ArgInfo {
    let mut arginfo = ArgInfo {
        max_size: u32::MAX,
        .. Default::default()
    };
    {
        //  This block limits scope of borrows by ap.refer() method
        use argparse::{ArgumentParser, Store}; // only visible here
        let mut ap = ArgumentParser::new();
        ap.set_description("Decoder for JPEG 2000 files.");
        ap.refer(&mut arginfo.in_url)
            .add_option(&["-i", "--infile"], Store, "Input URL or file.");
        ap.refer(&mut arginfo.out_file)
            .add_option(&["-o", "--outfile"], Store, "Output file.");
        ap.refer(&mut arginfo.max_size)
            .add_option(&["--maxsize"], Store, "Maximum dimension of output image");            
        ap.refer(&mut arginfo.verbose)
            .add_option(&["-v", "--verbose"], Store, "Verbose mode.");
        ap.refer(&mut arginfo.llsd_mode)
            .add_option(&["--llsd"], Store, "LLSD mode");
        ap.parse_args_or_exit();
    }
    //  Check for required args
    if !arginfo.llsd_mode {
        if arginfo.in_url.is_empty() || arginfo.out_file.is_empty() {
            eprintln!("If LLSD mode is off, an input URL and an output file must be specified");
            std::process::exit(1);
        }
    }
    arginfo
}

/// LLSD mode
fn run_llsd_mode(verbose: bool) -> Result<(), Error> {
    todo!()
}

/// Decompress one URL or file mode.
fn decompress_one_url(in_url: &str, out_file: &str, max_size: u32, verbose: bool) -> Result<(), Error> {
    // Initial dumb version.
    let jp2_image = Image::from_file(in_url)?; // load from file (not URL)
    let img: DynamicImage = (&jp2_image).try_into()?;  // convert
    img.save(out_file)?;            // save as PNG file
    Ok(())
}

/// Main program
fn main() {
    let args = parseargs();
    eprintln!("args: {:?}", args);               // ***TEMP***
    let status = if args.llsd_mode {
        run_llsd_mode(args.verbose)
    } else {
        decompress_one_url(args.in_url.as_str(), args.out_file.as_str(), args.max_size, args.verbose)
    };
    if let Err(e) = status {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
