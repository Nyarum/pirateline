use diesel::prelude::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::database::models;
use crate::server::handle;
use crate::server::income_packet;
use crate::server::outcome_packet;
use std::sync::{Arc, Mutex};

pub struct Server {
    pub db: Arc<Mutex<SqliteConnection>>,
}

impl Server {
    pub fn create_server(self: Arc<Self>, url: &str, port: &str) {
        let address = format!("{}:{}", url, port);

        println!(
            "{:?}",
            outcome_packet::pack(
                outcome_packet::Opcode::FirstDate,
                &outcome_packet::get_first_date().into_bytes()
            )
        );

        // Set the address and port for the server
        let listener = TcpListener::bind(&address).expect("Failed to bind to address");

        println!("Server listening on {}", address);

        // Accept incoming connections in a loop
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection established");

                    // Handle each connection in a new thread
                    let self_clone = Arc::clone(&self);
                    thread::spawn(move || {
                        self_clone.handle_client(stream);
                    });
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];

        let first_date = outcome_packet::pack(
            outcome_packet::Opcode::FirstDate,
            &outcome_packet::get_first_date().into_bytes(),
        );

        stream.write_all(&first_date).unwrap_or_else(|e| {
            println!("Failed to write to stream: {}", e);
            return;
        });

        loop {
            // Read data from the client
            let bytes_read = match stream.read(&mut buffer) {
                Ok(bytes_read) => bytes_read,
                Err(e) => {
                    println!("Failed to read from stream: {}", e);
                    return;
                }
            };

            if bytes_read == 0 {
                println!("Client disconnected");
                return;
            }

            println!("Received: {:?}", &buffer[..bytes_read]);

            if bytes_read == 2 {
                let array: [u8; 2] = [0, 2];
                stream.write(&array).unwrap_or_else(|e| {
                    println!("Failed to write to stream ping packet: {}", e);
                    0
                });
                continue;
            }

            let header = match income_packet::Header::unpack(&buffer[..bytes_read]) {
                Ok(header) => header,
                Err(e) => {
                    println!("Failed to unpack header: {}", e);
                    return;
                }
            };

            // Handle the header
            let packet = match header.handle(&buffer[8..bytes_read]) {
                Ok(packet) => packet,
                Err(e) => {
                    println!("Failed to handle packet: {}", e);
                    return;
                }
            };

            // Process the packet
            match packet {
                income_packet::Packet::Auth(auth) => {
                    let mut db = match self.db.lock() {
                        Ok(db) => db,
                        Err(e) => {
                            println!("Failed to get database connection: {}", e);
                            return;
                        }
                    };

                    auth.handle(&mut *db);
                }
                income_packet::Packet::Exit => {
                    if let Err(e) = stream.shutdown(std::net::Shutdown::Both) {
                        println!("Failed to shut down stream: {}", e);
                    } else {
                        println!("Exit");
                    }
                    return;
                }
            }

            let chars_output: [u8; 20] = [
                0x00, 0x00, 0x7C, 0x35, 0x09, 0x19, 0xB2, 0x50, 0xD3, 0x49, 0x00, 0x01, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x32, 0x14,
            ];

            let chars_packet =
                outcome_packet::pack(outcome_packet::Opcode::ResponseCharacters, &chars_output);

            stream.write(&chars_packet).unwrap_or_else(|e| {
                println!("Failed to write to stream: {}", e);
                0
            });

            println!("test");
        }
    }
}
