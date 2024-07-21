extern crate flate2;

// Loading the dependencies - relevant for compressing the files in the directory
use flate2::write::GzEncoder;
use flate2::Compress;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {}
