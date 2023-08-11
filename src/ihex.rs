use std::u8;
use std::u16;

#[derive(Debug)]
pub struct IHexLine {
    start_code: char,
    byte_count: u8,
    address: u16,
    record_type: RecordType,
    data: Vec<u8>,
    pub checksum: u8,
}

impl IHexLine {
    pub fn verify_checksum(&self) -> bool {
        let mut sum: u8 = 0;
        sum += self.byte_count;
        sum += self.address as u8;
        sum += (self.address >> 8) as u8;
        sum += self.record_type.into_u8();
        for data_byte in &self.data {
            sum += *data_byte; 
        }
        let checksum = sum.wrapping_neg();
    
        checksum == self.checksum
    }
}

#[derive(Debug)]
enum RecordType {
    Data,
    EndOfFile,
    ExtendedSegAddress,
    StartSegAddress,
    ExtendedLinAddress,
    StartLinAddress,
}

impl RecordType {
    fn into_u8(&self) -> u8 {
        match self {
            RecordType::Data => 0,
            RecordType::EndOfFile => 1,
            RecordType::ExtendedSegAddress => 2,
            RecordType::StartSegAddress => 3,
            RecordType::ExtendedLinAddress => 4,
            RecordType::StartLinAddress => 5,
        }
    }
}

pub fn parse_line(input_string: &str) -> Result<IHexLine,()> {

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
        parse_data_byte(&input_string[index..index+2], &mut data)?;
        index += 2;
    }

    // Checksum
    let checksum = parse_checksum(&input_string[index..index+2])?;

    let ihex_line = IHexLine {
        start_code,
        byte_count,
        address,
        record_type,
        data,
        checksum
    };


    Ok(ihex_line)
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

pub fn verify_checksum(ihex_line: &IHexLine) -> bool {
    let mut sum: u8 = 0;
    sum += ihex_line.byte_count;
    sum += ihex_line.address as u8;
    sum += (ihex_line.address >> 8) as u8;
    sum += ihex_line.record_type.into_u8();
    for data_byte in &ihex_line.data {
        sum += *data_byte; 
    }
    let checksum = sum.wrapping_neg();

    checksum == ihex_line.checksum
}
