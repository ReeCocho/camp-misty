use rand::Rng;

use crate::game::game_state::*;
use crate::util::*;
use crate::packets::*;

/// A server that hosts a game
pub struct Server
{
    /// Game state.
    state : GameState,

    /// TCP server listener.
    listener : std::net::TcpListener,

    /// Client stream.
    client : Option<std::net::TcpStream>
}

impl Server
{
    /// Constructor.
    pub fn new(port : u32) -> Result<Server, ServerError>
    {
        // Initialize TCP listener
        let port_as_str = port.to_string();
        match std::net::TcpListener::bind(String::from("0.0.0.0:") + port_as_str.as_str())
        {
            // Listener was created successfully. 
            Ok(listener) =>
            {
                return Ok(
                    Server
                    {
                        state : GameState::new(),
                        listener : listener,
                        client : None
                    }
                );
            }

            // Return an error saying the server was unable to be created.
            Err(_) =>
            {
                return Err(ServerError);
            }
        }
    }

    /// Wait for a client to connect
    pub fn wait_for_client(&mut self) -> Result<(), ConnectionError>
    {
        match self.listener.accept()
        {
            // Client connected successfully
            Ok((socket, _addr)) =>
            {
                self.client = Some(socket);
            }

            // There was an issue while a client tried to connect
            Err(_e) =>
            {
                return Err(ConnectionError);
            }
        }

        return Ok(());
    }

    /// Host game logic.
    pub fn host_game()
    {
        // Loop to make server
        let mut server : Server;
        loop
        {
            // Ask for port number
            println!("Please enter the port you wish to use.");

            // Loop to get port
            let port : u32;
            loop
            {
                // Convert to int
                match read_str().parse::<u32>()
                {
                    // Port is valid
                    Ok(i) =>
                    {
                        port = i;
                        break;
                    }

                    // There was a problem
                    Err(_) =>
                    {
                        println!("Sorry, I didn't understand you.");
                    }
                }
            }

            // Attempt to create server
            match Server::new(port)
            {
                // Created successfully
                Ok(s) =>
                {
                    server = s;
                    break;
                }

                // There was a problem
                Err(_) =>
                {
                    println!("There was a problem creating the server.");
                    println!("Would you like to (T)ry again or (R)eturn to the main menu?");

                    match pick_char(&vec!['T', 'R'], "Sorry, that isn't an option.")
                    {
                        'T' => {}
                        'R' => { return; }
                        _ => panic!("Invalid input.")
                    }
                }
            }
        }

        // Wait for client
        println!("Waiting for client...");
        match server.wait_for_client()
        {
            // Client joined successfully
            Ok(_) =>  {}
            Err(_) => 
            {
                println!("Woops! Looks like something went wrong when the client tried to connect. Returning to the main menu.");
                return;
            }
        }

        // Play the game
        println!("Client connected!");
        server.play();
    }

    /// Play the game!
    pub fn play(&mut self)
    {
        // Client must exist
        match &mut self.client
        {
            // Client exists
            Some(client) =>
            {
                // Host gets to choose if they want to be the killer or the victim
                println!("Would you like to be the (K)iller, the (V)ictim, or (R)andomly choose?");
                let player_type = match pick_char(&vec!['K', 'V', 'R'], "Sorry, that isn't a valid option.")
                {
                    'K' => PlayerType::Killer,
                    'V' => PlayerType::Victim,
                    'R' => if rand::thread_rng().gen_range(0, 2) == 0 { PlayerType::Killer } else { PlayerType::Victim }
                    _ => panic!("Invalid option chosen!")
                };

                // Tell the client what player type we are
                write_over_tcp::<PlayerType>(client, &player_type);

                // Determine rules based on player type
                match player_type
                {
                    // Victim logic
                    PlayerType::Victim =>
                    {
                        
                    }

                    // Killer logic
                    PlayerType::Killer =>
                    {

                    }
                }
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