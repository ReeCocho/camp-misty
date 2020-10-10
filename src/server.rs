use rand::Rng;

use crate::game::game_state::*;
use crate::game::victim::*;
use crate::game::killer::*;
use crate::game::sub_section::*;
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
        // Randomize game state
        self.state.gen_state();

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

                // Generate packet to send to client that describes the game state
                let mut state_packet = GameStatePacket::new();
                let mut state_packet_ind : usize = 0;
                for (i, section) in self.state.sections.iter().enumerate()
                {
                    for (j, sub_section) in section.sub_sections.iter().enumerate()
                    {
                        if sub_section.part != CarPart::None
                        {
                            state_packet.hidden_parts[state_packet_ind] = 
                            (
                                (i as u32, j as u32),
                                sub_section.part
                            );
                            state_packet_ind += 1;
                        }
                    }
                }

                // Send client the game state
                write_over_tcp::<GameStatePacket>(client, &state_packet);

                // Game loop
                loop
                {
                    // Play game
                    let our_move = 
                    if player_type == PlayerType::Killer 
                    { play_killer(&mut self.state) } 
                    else 
                    { play_victim(&mut self.state) };

                    // Send our move to the client
                    write_over_tcp::<MovePacket>(client, &MovePacket { 0 : our_move.0 as u32, 1 : our_move.1 as u32 });

                    // Wait for client to tell us their move
                    println!("Waiting for the other player move...");
                    let client_move = read_over_tcp::<MovePacket>(client);

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
                        // If we are the victim, tell the client where the trap was placed
                        if player_type == PlayerType::Victim
                        {
                            // Place the trap
                            let trap_loc = victim_place_trap(&mut self.state) as TrapPacket;

                            // Tell the client where we placed the trap
                            write_over_tcp::<TrapPacket>(client, &trap_loc);
                        }
                        // If we are the killer, have the client tell us where they placed the trap
                        else
                        {
                            let trap_loc = read_over_tcp::<TrapPacket>(client);
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