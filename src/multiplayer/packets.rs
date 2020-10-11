use crate::game::game_state::*;
use byteorder::{ByteOrder, LittleEndian};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;

/// Function to write the contents of a structure over a TCP connection.
pub fn write_over_tcp<T>(stream: &mut std::net::TcpStream, val: &T)
where
    T: Serialize,
{
    // Convert the data to a JSON string
    let serialized = serde_json::to_string(val).expect("Serialization of structure failed!");

    // Create a buffer to hold size of serialized data and the data itself
    let mut data = Vec::<u8>::with_capacity(4 + serialized.len());
    data.resize(4, 0);

    // Write size of serialized data to a buffer
    LittleEndian::write_u32(&mut data, serialized.len() as u32);

    // Write serialized data to buffer
    data.extend_from_slice(serialized.as_bytes());

    // Write data to TCP stream.
    stream.write_all(&data).expect("Error writing data to TCP stream!");
}

/// Function to read the contents of a structure from a TCP stream.
pub fn read_over_tcp<T: serde::de::DeserializeOwned>(stream: &mut std::net::TcpStream) -> T {
    // Read size of structure
    let mut buf = [0u8; 4];
    let mut pos: usize = 0;
    while pos < 4 {
        match stream.read(&mut buf[pos..]) {
            Ok(n) => {
                pos += n;
            }
            Err(_) => panic!("Error reading data over TCP stream!"),
        }
    }

    // Create new buffer to hold serialized data
    let buf_size = LittleEndian::read_u32(&buf) as usize;
    let mut buf = vec![0; buf_size];

    // Read serialized data
    let mut pos: usize = 0;
    while pos < buf_size {
        match stream.read(&mut buf[pos..]) {
            Ok(n) => {
                pos += n;
            }
            Err(_) => panic!("Error reading data over TCP stream!"),
        }
    }

    // Deserialize data
    serde_json::from_slice::<T>(&buf).expect("Unable to deserialized structure!")
}

/// An enum used to identify a type of player (either a victim or killer)
#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum PlayerType {
    Killer,
    Victim,
}

/// A structure used to describe the state of the game to a client.
#[derive(Serialize, Deserialize)]
pub struct GameStatePacket {
    // List of the spots the car parts are hidden in.
    pub hidden_parts: [(u32, u32); SECTION_COUNT],
}

impl Default for GameStatePacket {
    fn default() -> Self {
        Self::new()
    }
}

impl GameStatePacket {
    /// Constructor.
    pub fn new() -> GameStatePacket {
        GameStatePacket {
            hidden_parts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
        }
    }
}

/// A structure used to communicate the move made during the game.
#[derive(Serialize, Deserialize)]
pub struct MovePacket(pub u32, pub u32);

/// A trap used to communicate where the victim has placed a trap.
pub type TrapPacket = u32;
