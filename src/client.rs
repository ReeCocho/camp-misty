use crate::game::game_state::*;
use crate::util::*;
use crate::packets::*;
use crate::game::victim::*;
use crate::game::killer::*;
use crate::game::sub_section::*;

/// A client that joins a hosts game
pub struct Client
{
    /// Game state.
    state : GameState,

    /// Server stream.
    server : std::net::TcpStream
}

impl Client
{
    /// Create a new client stream.
    /// 
    /// There are two parameters. The first is a string corresponding to the 
    /// ipaddress to connect to. The other is the port number to connect using.
    pub fn new(ip : &str, port : u32) -> Result<Client, ClientError>
    {
        // Convert port to a string
        let port_as_str = port.to_string();

        // Try to create stream
        match std::net::TcpStream::connect(String::from(ip) + ":" + port_as_str.as_str())
        {
            // Stream created successfully
            Ok(stream) =>
            {
                return Ok(
                    Client
                    {
                        state : GameState::new(),
                        server : stream
                    }
                )
            }

            // Something went wrong
            Err(_) =>
            {
                return Err(ClientError);
            }
        }
    }

    /// Join a game.
    pub fn join_game()
    {
        // Loop to create client
        let mut client : Client;
        loop
        {
            // Ask for IP
            println!("Please enter the IP address of the host.");
            let ip = read_str();

            // Ask for port number
            println!("Please enter the port of the host.");

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

            // Attempt to create client
            println!("Attempting to join {}:{}...", ip, port);
            match Client::new(ip.as_str(), port)
            {
                // Created successfully
                Ok(c) =>
                {
                    client = c;
                    break;
                }

                // There was a problem
                Err(_) =>
                {
                    println!("There was a problem joining the host.");
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

        // Play the game
        println!("Connected!");
        client.play();
    }

    /// Play the game!
    pub fn play(&mut self)
    {
        // Determine what player type we are
        println!("Waiting for host to choose player type...");

        let mut player_type : PlayerType;
        match read_over_tcp::<PlayerType>(&mut self.server)
        {
            PlayerType::Killer => 
            { 
                println!("You are the victim!"); 
                player_type = PlayerType::Victim; 
            }

            PlayerType::Victim => 
            {
                println!("You are the killer!");
                player_type = PlayerType::Killer; 
            }
        }

        // Read game state
        let loaded_state = read_over_tcp::<GameStatePacket>(&mut self.server);

        // Update our state with new state
        for part in &loaded_state.hidden_parts
        {
            self.state.hide_part(part.0.0 as usize, part.0.1 as usize, part.1);
        }

        // Game loop
        loop
        {
            // Play game
            let our_move = 
            if player_type == PlayerType::Killer 
            { play_killer(&mut self.state) } 
            else 
            { play_victim(&mut self.state) };

            // Send our move to the server
            write_over_tcp::<MovePacket>(&mut self.server, &MovePacket { 0 : our_move.0 as u32, 1 : our_move.1 as u32 });

            // Wait for client to tell us their move
            println!("Waiting for other player to move...");
            let client_move = read_over_tcp::<MovePacket>(&mut self.server);

            // Submit moves to the game state
            let mut res : (RoundResult, (CarPart, usize));
            if player_type == PlayerType::Killer 
            {
                res = self.state.play(
                    (client_move.0 as usize, client_move.1 as usize),
                    our_move
                ).expect("Something went wrong during play");
            }
            else
            {
                res = self.state.play(
                    our_move, 
                    (client_move.0 as usize, client_move.1 as usize)
                ).expect("Something went wrong during play");
            }

            // Place trap if evaded
            if res.0 == RoundResult::Evaded
            {
                // If we are the victim, place the trap and tell the server about it
                if player_type == PlayerType::Victim
                {
                    // Place the trap
                    let trap_loc = victim_place_trap(&mut self.state) as TrapPacket;

                    // Tell the client where we placed the trap
                    write_over_tcp::<TrapPacket>(&mut self.server, &trap_loc);
                }
                // If we are the killer, wait for the server to tell us where they placed the trap
                else
                {
                    let trap_loc = read_over_tcp::<TrapPacket>(&mut self.server);
                    self.state.place_trap(trap_loc as usize);
                }
            }
            // Break if someone won
            else if res.0 == RoundResult::Caught
            {
                if player_type == PlayerType::Victim
                {
                    println!("Noooo!!! The killer slices your back and you fall dead... You lose!");
                }
                else
                {
                    println!("Muahahahaha!!! You slice the victim across their back, and they fall dead... You win!");
                }

                break;
            }
            else if res.0 == RoundResult::AllPartsFound
            {
                if player_type == PlayerType::Victim
                {
                    println!("Yes!!! You found all of the car parts and are able to escape Camp Misty! You win!");
                }
                else
                {
                    println!("No!!! The victim found all the car parts and escaped Camp Misty!");
                }

                break;
            }
        }
    }
}



/// Error that might be thrown if there was an issue creating a client.
#[derive(Debug)]
pub struct ClientError;