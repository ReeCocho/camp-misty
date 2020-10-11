use super::net_play::*;
use super::packets::*;
use crate::game::game_state::*;
use crate::util::*;

/// A client that joins a hosts game
pub struct Client {
    /// Game state.
    state: GameState,

    /// Server stream.
    server: std::net::TcpStream,
}

impl Client {
    /// Create a new client stream.
    ///
    /// There are two parameters. The first is a string corresponding to the
    /// ipaddress to connect to. The other is the port number to connect using.
    pub fn new(ip: &str, port: u32) -> Result<Client, ClientError> {
        // Convert port to a string
        let port_as_str = port.to_string();

        // Try to create stream
        match std::net::TcpStream::connect(String::from(ip) + ":" + port_as_str.as_str()) {
            // Stream created successfully
            Ok(stream) => Ok(Client {
                state: GameState::new(),
                server: stream,
            }),

            // Something went wrong
            Err(_) => Err(ClientError),
        }
    }

    /// Join a game.
    pub fn join_game() {
        // Loop to create client
        let mut client: Client;
        loop {
            // Ask for IP
            println!("Please enter the IP address of the host.");
            let ip = read_str();

            // Ask for port number
            println!("Please enter the port of the host.");

            // Loop to get port
            let port: u32;
            loop {
                // Convert to int
                match read_str().parse::<u32>() {
                    // Port is valid
                    Ok(i) => {
                        port = i;
                        break;
                    }

                    // There was a problem
                    Err(_) => {
                        println!("Sorry, I didn't understand you.");
                    }
                }
            }

            // Attempt to create client
            println!("Attempting to join {}:{}...", ip, port);
            match Client::new(ip.as_str(), port) {
                // Created successfully
                Ok(c) => {
                    client = c;
                    break;
                }

                // There was a problem
                Err(_) => {
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
