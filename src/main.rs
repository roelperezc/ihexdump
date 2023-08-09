
use std::io;
use std::fs;
use clap::Parser;

mod input;
use input::Config;
use input::parse_input;

fn main() {

    let config = Config::parse();
    
    let mut buffer: Box<dyn io::Read> = match config.filename {
        Some(ref filename) => Box::new(fs::File::open(&filename).unwrap()),
        None => Box::new(io::stdin())
    };

    parse_input(buffer, config);

}


