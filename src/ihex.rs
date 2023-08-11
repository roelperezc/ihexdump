use std::u8;
use std::u16;

struct IHexLine {
    start_code: char,
    byte_count: u8,
    address: u16,
    record_type: RecordType,
    data: Vec<u8>,
    checksum: u8,
}


enum RecordType {
    Data,
    EndOfFile,
    ExtendedSegAddress,
    StartSegAddress,
    ExtendedLinAddress,
    StartLinAddress,
}

pub fn parse_line(input: String) -> Result<String,()> {

    let input_string = input.as_str();

    // Start code (:)
    // TODO: Accept other start codes
    let mut index = parse_start_code(input_string)?;
    let start_code = input_string.chars().nth(0).unwrap();

    index += 1;

    // Byte count
    let byte_count = parse_byte_count(&input_string[index..index+2])?;

    index += 2;

    // Address
    let address = parse_address(&input_string[index..index+4])?;

    index += 4;

    // Record type
    let record_type = parse_record_type(&input_string[index..index+2])?;

    index += 2;

    // Data
    let mut data: Vec<u8> = vec![];
    for _ in 0..byte_count {
        parse_data_byte(&input_string[index..index+1], &mut data)?;
        index += 1;
    }

    // Checksum
    let checksum = parse_checksum(&input_string[index..index+1])?;

    let ihex_line = IHexLine {
        start_code,
        byte_count,
        address,
        record_type,
        data,
        checksum
    };


    Ok(input)
}


fn parse_start_code(input_string: &str) -> Result<usize,()> {

    match input_string.find(':') {
        Some(index) => Ok(index),
        None => Err(()),
    }

}

fn parse_byte_count(input_string_byte_count: &str) -> Result<u8,()> {
    match u8::from_str_radix(input_string_byte_count, 16) {
        Ok(bc) => Ok(bc),
        Err(_) => Err(())
    }
}

fn parse_address(input_string_byte_count: &str) -> Result<u16,()> {
    match u16::from_str_radix(input_string_byte_count, 16) {
        Ok(address) => Ok(address),
        Err(_) => Err(())
    }
}

fn parse_record_type(input_string_record_type: &str) -> Result<RecordType,()> {
    match u8::from_str_radix(input_string_record_type, 16) {
        Ok(rec_type_num) => {
            match rec_type_num {
                0 => Ok(RecordType::Data),
                1 => Ok(RecordType::EndOfFile),
                2 => Ok(RecordType::ExtendedSegAddress),
                3 => Ok(RecordType::StartSegAddress),
                4 => Ok(RecordType::ExtendedLinAddress),
                5 => Ok(RecordType::StartLinAddress),
                _ => Err(())
            }
        },
        Err(_) => Err(())
    }

}

fn parse_data_byte(input_string_data_byte: &str, data: &mut Vec<u8>) -> Result<(),()> {
    match u8::from_str_radix(input_string_data_byte, 16) {
        Ok(byte) => { data.push(byte); Ok(()) }, 
        Err(_) => Err(())
    }
}

fn parse_checksum(input_string_checksum: &str) -> Result<u8,()> {
    match u8::from_str_radix(input_string_checksum, 16) {
        Ok(bc) => Ok(bc),
        Err(_) => Err(())
    }
}

fn verify_checksum(ihex_line: IHexLine) {
    ()
}