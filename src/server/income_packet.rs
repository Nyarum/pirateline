use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::io::{Cursor, Read};

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
    pub fn unpack(data: &[u8]) -> Result<Self, String> {
        if data.len() < 8 {
            return Err("Not enough bytes to unpack Header".to_string());
        }

        let mut cursor = Cursor::new(data);

        // Read fields in the correct order and handle errors
        let length = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let id = cursor
            .read_u32::<LittleEndian>()
            .map_err(|e| e.to_string())?;
        let opcode = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;

        Ok(Header { length, id, opcode })
    }

    pub fn handle(self: Self, data: &[u8]) -> Result<Packet, String> {
        match Opcode::try_from(self.opcode) {
            Ok(opcode) => match opcode {
                Opcode::Auth => {
                    match Auth::unpack(data) {
                        Ok(auth) => Ok(Packet::Auth(auth)),
                        Err(e) => Err(e),
                    }
                }
                Opcode::Exit => Ok(Packet::Exit),
                _ => Err(format!("Unsupported opcode: {:?}", opcode)),
            },
            Err(e) => {
                println!("Failed to unpack header: {}", e);
                return Err("Failed to unpack header".to_string());
            }
        }
    }
}

#[derive(Debug)]
pub struct Auth {
    pub key: Vec<u8>,
    pub login: String,
    pub password: Vec<u8>,
    pub mac: String,
    pub is_cheat: u16,
    pub client_version: u16,
}


impl Auth {
    fn unpack(data: &[u8]) -> Result<Self, String> {
        let mut cursor = Cursor::new(data);

        // Read fields in the correct order and handle errors
        let key_len = cursor
            .read_u16::<BigEndian>()
            .map_err(|e| e.to_string())?;

        let mut key = vec![0; key_len as usize];
        cursor
            .read_exact(key.as_mut_slice())
            .map_err(|e| e.to_string())?;

        let login_len = cursor
            .read_u16::<BigEndian>()
            .map_err(|e| e.to_string())?;
        let mut login_buf = vec![0; login_len as usize];
        cursor
            .read_exact(login_buf.as_mut_slice())
            .map_err(|e| e.to_string())?;

        let password_len = cursor
            .read_u16::<BigEndian>()
            .map_err(|e| e.to_string())?;
        let mut password = vec![0; password_len as usize];
        cursor
            .read_exact(password.as_mut_slice())
            .map_err(|e| e.to_string())?;

        let mac_len = cursor
            .read_u16::<BigEndian>()
            .map_err(|e| e.to_string())?;
        let mut mac_buf = vec![0; mac_len as usize];
        cursor
            .read_exact(mac_buf.as_mut_slice())
            .map_err(|e| e.to_string())?;

        let is_cheat = cursor
            .read_u16::<BigEndian>()
            .map_err(|e| e.to_string())?;
        let client_version = cursor
            .read_u16::<BigEndian>()
            .map_err(|e| e.to_string())?;

        let login = String::from_utf8(login_buf).map_err(|e| e.to_string())?;
        let mac = String::from_utf8(mac_buf).map_err(|e| e.to_string())?;

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
