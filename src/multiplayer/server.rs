use rand::Rng;

use super::net_play::*;
use super::packets::*;
use crate::game::game_state::*;
use crate::util::*;

/// A server that hosts a game
pub struct Server {
    /// Game state.
    state: GameState,

    /// TCP server listener.
    listener: std::net::TcpListener,

    /// Client stream.
    client: Option<std::net::TcpStream>,
}

impl Server {
    /// Constructor.
    pub fn new(port: u16) -> Result<Server, ServerError> {
        match std::net::TcpListener::bind(std::net::SocketAddr::from(([0, 0, 0, 0], port))) {
            Ok(listener) => Ok(Server {
                state: GameState::new(),
                listener,
                client: None,
            }),

            Err(_) => Err(ServerError),
        }
    }

    /// Wait for a client to connect
    pub fn wait_for_client(&mut self) -> Result<(), ConnectionError> {
        match self.listener.accept() {
            Ok((socket, _addr)) => {
                self.client = Some(socket);
                Ok(())
            }

            Err(_e) => Err(ConnectionError),
        }
    }

    /// Host game logic.
    pub fn host_game() {
        // Loop to make server
        let mut server: Server;
        loop {
            // Ask for port number
            println!("Please enter the port you wish to use.");

            // Loop to get port
            let port: u16;
            loop {
                // Try to convert to int
                match read_str().parse::<u16>() {
                    Ok(i) => {
                        port = i;
                        break;
                    }

                    Err(_) => {
                        println!("Sorry, I didn't understand you.");
                    }
                }
            }

            // Attempt to create server
            match Server::new(port) {
                Ok(s) => {
                    server = s;
                    break;
                }

                Err(_) => {
                    println!("There was a problem creating the server.");
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

        // Wait for client
        println!("Waiting for client...");
        match server.wait_for_client() {
            Ok(_) => {}
            Err(_) => {
                println!("Woops! Looks like something went wrong when the client tried to connect. Returning to the main menu.");
                return;
            }
        }

        // Play the game
        println!("Client connected!");
        server.play();
    }

    /// Play the game!
    pub fn play(&mut self) {
        // Randomize game state
        self.state.gen_state();

        // Client must exist
        match &mut self.client {
            Some(client) => {
                // Host gets to choose if they want to be the killer or the victim
                println!("Would you like to be the (K)iller, the (V)ictim, or (R)andomly choose?");
                let player_type =
                    match pick_char(&['K', 'V', 'R'], "Sorry, that isn't a valid option.") {
                        'K' => PlayerType::Killer,
                        'V' => PlayerType::Victim,
                        'R' => {
                            if rand::thread_rng().gen_range(0, 2) == 0 {
                                PlayerType::Killer
                            } else {
                                PlayerType::Victim
                            }
                        }
                        _ => panic!("Invalid option chosen!"),
                    };

                // Tell the client what player type we are
                write_over_tcp::<PlayerType>(client, &player_type);

                // Generate packet to send to client that describes the game state
                let mut state_packet = GameStatePacket::new();
                let mut state_packet_ind: usize = 0;
                for (i, section) in self.state.sections.iter().enumerate() {
                    for (j, sub_section) in section.sub_sections.iter().enumerate() {
                        if sub_section.part {
                            state_packet.hidden_parts[state_packet_ind] = (i as u32, j as u32);
                            state_packet_ind += 1;
                        }
                    }
                }

                // Send client the game state
                write_over_tcp::<GameStatePacket>(client, &state_packet);

                // Game loop
                while !net_play(player_type, &mut self.state, client) {}

                // Return to title screen
                println!("Enter anything to return to the title screen...");
                read_str();
            }

            // No client loaded
            None => {}
        }
    }
}

/// Server initialization error.
#[derive(Debug)]
pub struct ServerError;

/// Client connection error.
#[derive(Debug)]
pub struct ConnectionError;
