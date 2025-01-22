use chess_engine;

mod game;
use game::ChessServer;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 8000;

fn main() {

    let message = format!("serving on port {PORT}");

    match ChessServer::new(HOST, PORT) {
        Ok(mut server) => {
            server.serve_forever(message.as_str());
        },
        Err(error) => println!("chess server error: {error}")
    }

}