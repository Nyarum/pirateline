use crate::server::handle;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use diesel::prelude::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::io::{self, Cursor, Read};
use std::net::{TcpListener, TcpStream};

use my_proc_macro::Unpack;
use std::io::Error;

/// Reads a `u16` value from the cursor.
fn read_u16_be(cursor: &mut Cursor<&[u8]>) -> Result<u16, String> {
    cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())
}

/// Reads a byte vector of a specific length from the cursor.
fn read_bytes(cursor: &mut Cursor<&[u8]>, len: usize) -> Result<Vec<u8>, String> {
    let mut buf = vec![0; len];
    cursor.read_exact(&mut buf).map_err(|e| e.to_string())?;
    Ok(buf)
}

/// Reads a UTF-8 string from a byte vector.
fn read_utf8_string(buf: Vec<u8>) -> Result<String, String> {
    String::from_utf8(buf).map_err(|e| e.to_string())
}

#[derive(Debug)]
pub enum Packet {
    Auth(Auth),
    Exit,
}

#[derive(Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)] // Define the underlying integer type for the enum
pub enum Opcode {
    Auth = 431,
    Exit = 432,
    CreatePincode = 346,
    CreateCharacter = 435,
    RemoveCharacter = 436,
    UpdatePincode = 347,
}

pub struct Header {
    length: u16,
    id: u32,
    pub opcode: u16,
}

impl Header {
    // Function to unpack raw bytes into a Header struct
    pub fn unpack(data: &[u8]) -> Result<Self, Error> {
        if data.len() < 8 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough bytes to unpack Header",
            ));
        }

        let mut cursor = Cursor::new(data);

        // Read fields in the correct order and handle errors
        let length = cursor.read_u16::<BigEndian>()?;
        let id = cursor.read_u32::<LittleEndian>()?;
        let opcode = cursor.read_u16::<BigEndian>()?;

        Ok(Header { length, id, opcode })
    }

    pub fn handle(
        self: Self,
        stream: &mut TcpStream,
        db: &mut SqliteConnection,
        data: &[u8],
    ) -> Result<i32, String> {
        let opcode = Opcode::try_from(self.opcode).map_err(|e| e.to_string())?;

        match opcode {
            Opcode::Auth => {
                let auth = Auth::unpack(data).map_err(|e| e)?;
                auth.handle(db);
                return Ok(0);
            }
            Opcode::Exit => {
                let _ = stream.shutdown(std::net::Shutdown::Both);
                Ok(0)
            }
            _ => return Err(format!("Unsupported opcode: {:?}", opcode)),
        }
    }
}

#[derive(Debug)]
pub struct Auth {
    key: Vec<u8>,
    login: String,
    password: Vec<u8>,
    mac: String,
    is_cheat: u16,
    client_version: u16,
}

impl Auth {
    fn unpack(data: &[u8]) -> Result<Self, String> {
        let mut cursor = Cursor::new(data);

        // Read fields using utility functions
        let key_len = read_u16_be(&mut cursor)?;
        let key = read_bytes(&mut cursor, key_len as usize)?;

        let login_len = read_u16_be(&mut cursor)?;
        let login_buf = read_bytes(&mut cursor, login_len as usize)?;

        let password_len = read_u16_be(&mut cursor)?;
        let password = read_bytes(&mut cursor, password_len as usize)?;

        let mac_len = read_u16_be(&mut cursor)?;
        let mac_buf = read_bytes(&mut cursor, mac_len as usize)?;

        let is_cheat = read_u16_be(&mut cursor)?;
        let client_version = read_u16_be(&mut cursor)?;

        // Convert byte buffers to strings
        let login = read_utf8_string(login_buf)?;
        let mac = read_utf8_string(mac_buf)?;

        Ok(Auth {
            key,
            login,
            password,
            mac,
            is_cheat,
            client_version,
        })
    }
}

#[derive(Unpack)]
pub struct Test {
    pub test: String,
    pub test2: Vec<u8>,
    pub test3: u16,
}
