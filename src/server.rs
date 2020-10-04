use crate::game::game_state::*;

/// A server that hosts a game
struct Server
{
    /// Game state.
    state : GameState,

    /// TCP server listener.
    listener : std::net::TcpListener
}

impl Server
{
    /// Constructor.
    fn new(port : String)
    {

    }
}



/// Server initialization error.
#[derive(Debug, Clone)]
struct ServerError;