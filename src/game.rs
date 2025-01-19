use std::net::{ TcpListener, TcpStream, SocketAddr, ToSocketAddrs };
use std::io::Read;

use chess_engine::{ ChessMove, ChessBoard, ChessPieceColor };

#[derive(Debug)]
pub struct ChessPacket {
    data: [u8; 2], // move is a u16 but sockets send u8
    turn: u8 // turn number
}

#[derive(PartialEq)]
pub struct ChessClient<'a> {
    board: ChessBoard,
    client: &'a TcpStream
}

pub struct ChessServer<'a> {
    clients: [Option<&'a ChessClient>; 8], // only 8 clients at a time
    service: TcpListener,
}

impl ChessPacket {
    pub fn new(turn: u8, data: ChessMove) -> Self {
        Self { turn, data: [(data.0 & 0xff) as u8, (data.0 >> 8) as u8] }
    }

    pub fn from_stream(stream: &mut TcpStream) -> Self {
        let mut buffer: [u8; 3] = [0; 3];
        
        match stream.read(&mut buffer) {
            Ok(_) => println!("{:#?}", buffer),
            Err(_) => println!("error!")
        };

        return Self { data: buffer[1..3].try_into().unwrap(), turn: buffer[0] }
    }
}

impl ChessClient {
    pub fn new(client: &mut TcpStream, color: ChessPieceColor) -> Self {
        Self { client: client, board: ChessBoard::new(color) }
    }
}

impl ChessServer<'_> {
    pub fn new(host: &str, port: u16) -> Result<Self, String> {
        let service = TcpListener::bind(format!("{}:{}", host, port));
        match service {
            Ok(service) => Ok( Self { service, clients: [None; 8]} ),
            Err(_) => Err(format!("couldn't bind to address {host}:{port}"))
        }
    }

    pub fn client_connected(&self, stream: &mut TcpStream) {
        let index = self.clients.iter()
            .enumerate()
            .find(|(_i, client)| **client == None)
            .expect("too many clients").0;
        
        println!("{}", index);
    }

    pub fn serve_forever(&self) {
        for stream in self.service.incoming() {
            match stream {
                Ok(mut stream) => self.client_connected(&mut stream),
                Err(error) => println!("error: {error}")
            }
        }
    }
}