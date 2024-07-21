extern crate flate2;

// Loading the dependencies - relevant for compressing the files in the directory
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::io::Read;
use std::time::Instant;

fn main() {
    if args().len() != 4 {
        eprintln!("Usage : `encode/decode` `source` `target`");
        return;
    }

    let choice = args().nth(1).unwrap().to_string();
    if choice == "encode" {
        println!("******* RUST FILE COMPRESSOR ******** ");
        println!("Encoding process initiated. Please wait..");
        let mut input_file = BufReader::new(File::open(args().nth(2).unwrap()).unwrap());
        let output_file = File::create(args().nth(3).unwrap()).unwrap();
        let mut encoder = GzEncoder::new(output_file, Compression::default());

        let start = Instant::now();
        copy(&mut input_file, &mut encoder).unwrap();
        let output_file = encoder.finish().unwrap();

        println!(
            "Input file : {:?} \t File size : {} bytes ",
            args().nth(2).unwrap(),
            input_file.get_ref().metadata().unwrap().len()
        );
        println!(
            "Output file : {:?} \t Compressed file size : {} bytes",
            args().nth(3).unwrap(),
            output_file.metadata().unwrap().len()
        );
        println!("Task completed in {:?}", start.elapsed());
    } else if choice == "decode" {
        println!("******* RUST FILE COMPRESSOR ******** ");
        println!("Decoding process initiated. Please wait..");
        let input_file = BufReader::new(File::open(args().nth(2).unwrap()).unwrap());
        let mut write_output = File::create(args().nth(3).unwrap()).unwrap();
        let mut decoder = GzDecoder::new(input_file);

        let start = Instant::now();
        let mut buffer = Vec::new();
        match decoder.read_to_end(&mut buffer) {
            Ok(x) => {
                println!("Processed successfully : {}", x);
            }
            Err(e) => {
                println!("Something wrong while converting the format : ERR => {}", e);
            }
        };
        match copy(&mut decoder, &mut write_output) {
            Ok(_) => {
                println!("Data written to file");
            }
            Err(e) => {
                println!("Something wrong while writing data : ERR => {:?}", e);
            }
        }

        println!(
            "Output file : {:?} \t Decompressed file size : {} bytes",
            args().nth(3).unwrap(),
            write_output.metadata().unwrap().len()
        );
        println!("Task completed in {:?}", start.elapsed());
    }
}
