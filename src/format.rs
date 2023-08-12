use crate::ihex;

pub fn ihex_format(ihex_line: &ihex::IHexLine) -> String {

    match &ihex_line.record_type {
        ihex::RecordType::EndOfFile => return format!("EOF"),
        _ => ()
    }

    let mut line = address( ihex_line.address );
    line.push_str( output_data_hex( &ihex_line.data ).as_str() );
    line.push_str( output_data_ascii( &ihex_line.data ).as_str() );

    // TODO: Bad checksum output

    line

}

pub fn address(ihex_line_address: u16) -> String {
    format!(" {:04x}  ", ihex_line_address)
}

pub fn output_data_hex(ihex_line_data: &Vec<u8>) -> String {
    let mut data_string = String::new();

    for data_byte in ihex_line_data {
        data_string.push_str( format!("{:02x} ", data_byte).as_str() );
    }

    data_string
}

pub fn output_data_ascii(ihex_line_data: &Vec<u8>) -> String {
    let mut data_string = String::from(" |");

    for data_byte in ihex_line_data {

        data_string.push_str( (format!("{}", u8_as_char(*data_byte))).as_str() );
    }

    data_string.push('|');

    data_string
}


fn u8_as_char(byte: u8) -> char {
    if byte >= 33 && byte <= 126 {
        byte as char
    }
    else {
        '.'
    }
}