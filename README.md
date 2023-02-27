# jpeg2000-decoder
Decodes JPEG 2000 images in a subprocess, for safety.

# IN PROGRESS

# Overview

This project generates both a library and an executable.
The executable decodes JPEG 2000 images. 
The library, "jpeg2000-decoder", runs the executable in a subprocess.

This is intended primarily for Second Life / Open Simulator content.

# Executable

Usage: jpeg2000-decoder -i INFILE -o OUTFILE

## Options

* **-i INFILE** Input file, JPEG 2000. May be a URL or a file.
* **--input INFILE** 

* **-o OUTFILE** Output file. Only **.png** is currently supported.
* **--output OUTFILE**

* **--maxsize PIXELS** Maximum dimension of output image. Image will be fetched and reduced accordingly.

* **--llsd** Enables LLSD mode. The subprocess accepts commands and returns images, using Linden Lab Serial Data marshalling.

* **--user-agent USERAGENT** HTTP user agent to use when making requests.

* **-v** Verbose 
* **--verbose**

## LLSD mode

The executable accepts commands on standard input, and returns results on standard output.

Request format:
    
    let request: HashMap<String, LLSDValue> = [
        ("url".to_string(), LLSDValue::String("http://www.example.com/file.j2k".to_string())),
        ("maxsize".to_string(), LLSDValue::Integer(999)), // maximum size of returned image, largest dimension. Used to compute discard level.
        ("discard".to_string(), LLSDValue::Integer(2)), // requested image discard level. 0 is full size, 1 halves each dimension, etc.
    ];
   
Specify either **maxsize** or **discard**, but not both. Specifying **maxsize** allows getting an image of a specified resolution without knowing what
is available.
    
Reply format: 

    let reply: HashMap<String, LLSDValue> = [
        ("url".to_string(), LLSDValue::String("http://www.example.com/file.j2k".to_string())), // URL of request, for check
        ("err".to_string(), LLSDValue::String("Error message if any"), // if present, request failed.
        ("discard".to_string(), LLSDValue::Integer(2)), // returned image discard level. 0 is full size, 1 halves each dimension, etc.
        ("h".to_string(), LLSDValue::Integer(512)), // returned image height
        ("w".to_string(), LLSDValue::Integer(512)), // returned image width
        ("d".to_string(), LLSDValue::Integer(4),    // returned image depth (3 for RGB, 4 for RGBA)
        ("image".to_string(), LLSDValue::Binary(bytes), // returned image, raw bytes, no headers, size h * w * d
    ];

