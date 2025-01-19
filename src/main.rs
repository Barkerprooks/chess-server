use chess_engine;

mod game;
use game::{ChessServer, ChessClient, ChessPacket};

fn main() {
    ChessServer::new("localhost", 8000)
        .expect("could not create chess server")
        .serve_forever();
}