# Pirate King Online Server Emulator

This project is an **emulation** of the server-side logic for the MMO game **Pirate King Online**. It is implemented in Rust and provides foundational functionality for handling client-server communication, packet processing, and opcode management.

## Features

- **TCP Server**: Listens for incoming client connections and processes requests in separate threads.
- **Packet Handling**: Supports structured packing and unpacking of packets.
- **Opcode-Based Logic**: Implements various opcodes such as `Auth`, `Exit`, `CreateCharacter`, and more.
- **Extensibility**: Designed to be easily extended with new opcodes and packet types.
- **MMO-Specific Logic**: Built specifically to emulate the server-side mechanics of Pirate King Online.

## Project Structure

```plaintext
src/
├── main.rs                   // Entry point and server logic
├── server/
│   ├── income_packet.rs      // Incoming packet handling
│   ├── outcome_packet.rs     // Outgoing packet handling
