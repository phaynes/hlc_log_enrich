//! 
//! When processing log files across a wide area distributed cluster, it is ideal that:
//!   o All files are processed.
//!   o Once and only once.
//!   o Files are chunked to support re-try functions in contested environments.
//! 
//! This utility log files using the concept of a hybrid logical clock to ensure 
//! all records and files are unique.
//! 
//! The program:
//!    Reads a compressed log file
//!    Embelishes each record with unique timestamps per record;
//!    Writes it back out in a series of chunks to support managed distribution.
//! 

const MAX_LOG_LINES_PER_FILE = 10000000;

#[macro_use]
extern crate clap;

use hybrid_clocks::{Clock};
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

use std::path::Path;
use std::process;
use flate2::read::GzDecoder;

fn main() -> Result<()> {
    let matches = clap_app!(hlc_enrich =>
        (version: "1.0")
        (author: "Steve S. <s.smith@cricketaustralia.com.au>")
        (about: "HLC log file enricher.")
        (@arg input: -i  +takes_value +required "Input source")
        (@arg output: -o +takes_value +required "Output path")
        (@arg host: -h +takes_value +required "Hostname")        
    ).get_matches();
    
    let mut output_pathname = String::from(matches.value_of("output").unwrap());
    if !output_pathname.ends_with("/") {
        output_pathname.push('/');
    }
    
    let mut hostname = String::from(matches.value_of("host").unwrap());
    hostname.insert(0, '.');
    hostname.push('.');
    
    let mut hlc_clock = Clock::wall_ns().unwrap();
    let file_timestamp =  hlc_clock.now().unwrap();
    
    let input_filename  = matches.value_of("input").unwrap();
    let input_path = Path::new(input_filename);
    let mut output_file_header = format!("{:?}{}", input_path.file_name().unwrap(),file_timestamp);
    output_file_header = output_file_header.replace(".gz", &hostname);
    output_pathname.push_str(&output_file_header);
    
    println!("File name = {}", output_pathname);

    if !input_path.exists() {
        println!("Unable to find file {}", input_filename);
        process::exit(1);
    }    

    let mut tar_gz = File::open(&input_path)?;
    let decoder = GzDecoder::new(tar_gz);

    let mut line_counter = 0;
    
    for line in BufReader::new(decoder).lines() {
        let timestamp = hlc_clock.now().unwrap();

        println!("{},{}", timestamp, line.unwrap());
    }
    
    Ok(())

}
