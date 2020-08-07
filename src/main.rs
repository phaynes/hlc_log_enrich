//!
//! For log files processed across a wide area distributed cluster, it is ideal that they are:
//!   o All processed.
//!   o Once and only once.
//!   o Chunked to support re-try functions in contested environments without having to
//!     redo the entire file.
//!
//! Using the concept of a hybrid logical clock (HLC) to ensure all records and files are unique,
//! this program reads a compressed text log file and timestamps each record.
//!
//! The program:
//!    Reads a compressed log file
//!    Embelishes each record with unique timestamps per record;
//!    Writes it back out in a series of chunks to support managed distribution.
//!

const MAX_LOG_LINES_PER_FILE :i32 = 500000;

#[macro_use]
extern crate clap;

use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;

use hybrid_clocks::{Clock};

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::Write;
use std::path::Path;
use std::process;

//
// Read command line arguments, check source file path and kick off HLC transform.
//
fn main() {

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

    let input_filename  = matches.value_of("input").unwrap();
    let input_path = Path::new(input_filename);
     if !input_path.exists() {
        println!("Unable to find file {}", input_filename);
        process::exit(1);
    }

    // Create output file name.
    let output_short_filename = String::from(input_path.file_name().unwrap().to_str().unwrap());
    let mut hlc_clock = Clock::wall_ns().unwrap();
    let file_timestamp =  hlc_clock.now().unwrap();
    let mut output_file_header = format!("{}{}", output_short_filename, file_timestamp.time);
    output_file_header = output_file_header.replace(".gz", &hostname);
    output_pathname.push_str(&output_file_header);

    // Open existing file to read.
    let tar_gz = File::open(&input_path).unwrap();
    let mut decoder = GzDecoder::new(tar_gz);
    let mut file_chunk_counter = 0;

    while !writeout_records(&mut decoder, &output_pathname, file_chunk_counter) {
        file_chunk_counter += 1;
    }
}

//
// Write updated records to gz compressed file in chunks.
//
fn writeout_records(decoder : &mut GzDecoder<File>, output_pathname : &String, file_chunk_counter : i32) -> bool {

    let mut done = true;
    let mut hlc_clock = Clock::wall_ns().unwrap();
    let f = File::create(format!("{}.{}.gz", output_pathname, file_chunk_counter)).unwrap();
    let mut gz = GzEncoder::new(f, Compression::default());

    let mut line_counter = 0;
    for line in BufReader::new(decoder).lines() {
        let timestamp = hlc_clock.now().unwrap();
        let output_line = format!("{},{}\r\n", timestamp, line.unwrap());
        gz.write_all(output_line.as_bytes()).unwrap();
        line_counter += 1;
        if line_counter == MAX_LOG_LINES_PER_FILE {
            done = false;
            break;
        }
    }
    gz.finish().unwrap();
    done
}
