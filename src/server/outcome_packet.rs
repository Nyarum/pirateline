
use chrono::{Datelike, Timelike, Local};
use std::io::{Write, Cursor};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt}; // For network order

pub enum Opcode {
    FirstDate = 940,
    ResponseCharacters = 931,
}

pub fn pack(opcode: Opcode, data: &[u8]) -> Vec<u8> {
    let data_len = data.len();
    let mut res = Cursor::new(Vec::new());

    // Write the length in network byte order (Big Endian)
    res.write_u16::<BigEndian>((8 + data_len) as u16).expect("Failed to write length");

    // Write the fixed UInt32 value (128)
    res.write_u32::<LittleEndian>(128).expect("Failed to write fixed UInt32");

    // Write the opcode in network byte order (Big Endian)
    res.write_u16::<BigEndian>(opcode as u16).expect("Failed to write opcode");

    // Write the data
    res.write_all(data).expect("Failed to write data");

    // Return the resulting buffer
    res.into_inner()
}

pub fn get_first_date() -> String {
    let time_now = Local::now();

    format!(
        "[{:02}-{:02} {:02}:{:02}:{:02}:{:03}]",
        time_now.month(),
        time_now.day(),
        time_now.hour(),
        time_now.minute(),
        time_now.second(),
        time_now.timestamp_subsec_millis()
    )
}