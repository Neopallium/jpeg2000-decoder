//! # jpeg2000-decoder  -- Decoder program for JPEG 2000 files.
//!
//  Animats
//  April, 2021
//

////use url::Url;
////use std::path::Path;


/// Arguments to the program
#[derive(Clone, Debug, Default)]
struct ArgInfo {
    /// Source URL
    pub in_url: String,
    /// Destination file
    pub out_file: String,
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

/// Main program
fn main() {
    let args = parseargs();
    eprintln!("args: {:?}", args);               // ***TEMP***
}
