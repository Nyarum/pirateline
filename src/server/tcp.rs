use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::server::outcome_packet;
use crate::server::income_packet;


pub fn create_server(url: &str, port: &str) {
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
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
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
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
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

                match income_packet::Header::unpack(&buffer[..bytes_read]) {
                    Ok(header) => {
                        match header.handle(&buffer[8..bytes_read]) {
                            Ok(packet) => {
                                println!("Packet: {:?}", packet);

                                match packet {
                                    income_packet::Packet::Auth(auth) => {
                                        println!("Auth: {:?}", auth);
                                    }
                                    income_packet::Packet::Exit => {
                                        stream.shutdown(std::net::Shutdown::Both).unwrap();
                                        println!("Exit");
                                        return
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Failed to handle packet: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to unpack header: {}", e);
                        return;
                    }
                }
                

                let chars_output: [u8; 20] = [
                    0x00, 0x00, 0x7C, 0x35, 0x09, 0x19, 0xB2, 0x50, 0xD3, 0x49, 0x00, 0x01, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x14,
                ];

                let chars_packet = outcome_packet::pack(
                    outcome_packet::Opcode::ResponseCharacters,
                    &chars_output,
                );
            
                stream.write(&chars_packet).unwrap_or_else(|e| {
                    println!("Failed to write to stream: {}", e);
                    0
                });

                println!("test");
            }
            Err(e) => {
                println!("Failed to read from stream: {}", e);
                return
            }
        }
    }
}
