use crate::game::game_state::*;

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
        match std::net::TcpListener::bind(String::from("127.0.0.1:") + port_as_str.as_str())
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
}



/// Server initialization error.
#[derive(Debug)]
pub struct ServerError;

/// Client connection error.
#[derive(Debug)]
pub struct ConnectionError;