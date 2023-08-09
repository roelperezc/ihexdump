use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short='F', long="filename", value_name = None)]
    pub filename: Option<String>,
    #[arg(short='A', long="address", default_value_t = false)]
    pub address: bool,
    #[arg(short='C', long="ascii", default_value_t = false)]
    pub ascii: bool,
}


pub fn parse_input(
    mut buffer: Box<dyn io::Read>,
    config: Config) {

    let address = match config.address {
        true => {()},
        false => {()}
    };

    let ascii = match config.ascii {
        true => {()},
        false => {()}
    };

    loop {
        match read_line(&mut buffer) {
            Ok(line) => output_line(line),
            Err(_)=> return
        }
    }

}

fn read_line(buffer: &mut Box<dyn io::Read>) -> Result<String,()> {
    let mut c : [u8;1] = [0];
    let mut string = String::new();

    buffer.read(&mut c);
    while c[0] != b'\n' && c[0] != 0 {
        string.push(c[0] as char);
        buffer.read(&mut c);
    }

    if c[0] == 0 {
        return Err(());
    }

    Ok(string)
}

fn output_line(input_line: String) {
    println!("{}", input_line);
}