use std::net::ToSocketAddrs;

use super::net_play::*;
use super::packets::*;
use crate::game::game_state::*;
use crate::util::*;

/// A client that joins a hosts game.
pub struct Client {
    /// Game state.
    state: GameState,

    /// Server stream.
    server: std::net::TcpStream,
}

impl Client {
    /// Create a new client stream.
    ///
    /// The only argument is the address to connect to.
    pub fn new(addr: &std::net::SocketAddr) -> Result<Client, ClientError> {
        match std::net::TcpStream::connect(addr) {
            Ok(stream) => Ok(Client {
                state: GameState::new(),
                server: stream,
            }),

            Err(_) => Err(ClientError),
        }
    }

    /// Join a game.
    pub fn join_game() {
        // Loop to create client
        let mut client: Client;
        loop {
            // Ask for address of host
            println!("Please enter the address of the host.");
            println!("Use the format \"IP:PORT\".");

            let addr = match read_str().to_socket_addrs() {
                Ok(mut addr_list) => {
                    // Only take the first address
                    if let Some(addr) = addr_list.next() {
                        addr
                    } else {
                        println!("Sorry, I couldn't understand you.");
                        continue;
                    }
                }

                Err(_) => {
                    println!("Sorry, I couldn't understand you.");
                    continue;
                }
            };

            // Attempt to create client
            println!("Attempting to join connect to {}...", addr);
            match Client::new(&addr) {
                Ok(c) => {
                    client = c;
                    break;
                }

                Err(_) => {
                    // Option to try again or quit to main menu
                    println!("There was a problem joining the host.");
                    println!("Would you like to (T)ry again or (R)eturn to the main menu?");

                    match pick_char(&['T', 'R'], "Sorry, that isn't an option.") {
                        'T' => {}
                        'R' => {
                            return;
                        }
                        _ => panic!("Invalid input."),
                    }
                }
            }
        }

        // Play the game
        println!("Connected!");
        client.play();
    }

    /// Play the game!
    pub fn play(&mut self) {
        // Determine what player type we are
        println!("Waiting for host to choose player type...");

        // The server tells us what their player type is, so ours is the opposite
        let player_type: PlayerType;
        match read_over_tcp::<PlayerType>(&mut self.server) {
            PlayerType::Killer => {
                println!("You are the victim!");
                player_type = PlayerType::Victim;
            }

            PlayerType::Victim => {
                println!("You are the killer!");
                player_type = PlayerType::Killer;
            }
        }

        // Read game state
        let loaded_state = read_over_tcp::<GameStatePacket>(&mut self.server);

        // Update our state with new state
        for part in &loaded_state.hidden_parts {
            self.state.hide_part(part.0 as usize, part.1 as usize);
        }

        // Game loop
        while !net_play(player_type, &mut self.state, &mut self.server) {}

        // Return to title
        println!("Enter anything to return to the title screen...");
        read_str();
    }
}

/// Error that might be thrown if there was an issue creating a client.
#[derive(Debug)]
pub struct ClientError;
