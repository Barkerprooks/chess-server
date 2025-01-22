use chess_engine;

mod game;
use game::ChessServer;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 8000;

fn main() {

    match ChessServer::new(HOST, PORT) {
        Ok(mut server) => {
            println!("serving on port {PORT}");
            server.serve_chess_games();
        },
        Err(error) => println!("chess server error: {error}")
    }

}