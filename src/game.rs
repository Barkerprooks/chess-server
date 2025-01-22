use std::net::{ TcpListener, TcpStream, SocketAddr, ToSocketAddrs };
use std::io::{ Read, Write };

use chess_engine::{ ChessMove, ChessBoard, ChessPieceColor };

const INITIAL_CLIENTS: [Option<ChessClient>; 8] = [ None, None, None, None, None, None, None, None ];
const ACCEPT_MAX_CLIENTS: &str = "too many clients playing at the moment";

#[derive(Debug)]
pub struct ChessPacket {
    data: [u8; 2], // move is a u16 but sockets send u8
    turn: u8 // turn number
}

#[derive(Debug)]
pub struct ChessClient {
    board: ChessBoard,
    client: TcpStream
}

pub struct ChessServer {
    clients: [Option<ChessClient>; 8], // only 8 clients at a time
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
    pub fn new(client: TcpStream, color: ChessPieceColor) -> Self {
        Self { client: client, board: ChessBoard::new(color) }
    }
}

impl ChessServer {
    pub fn new(host: &str, port: u16) -> Result<Self, String> {
        match TcpListener::bind(format!("{}:{}", host, port)) {
            Ok(service) => Ok( Self { clients: INITIAL_CLIENTS, service }),
            Err(error) => Err(error.to_string())
        }
    }

    pub fn serve_forever(&mut self, message: &str) {
        println!("{message}");

        for stream in self.service.incoming() {
            match stream {
                Ok(mut stream) => {
                    let index = self.clients.iter()
                        .enumerate()
                        .find(|(_, client)| !client.is_some())
                        .unwrap_or((usize::MAX, &None))
                        .0;

                    if index == usize::MAX {
                        stream.write(&ACCEPT_MAX_CLIENTS.as_bytes())
                            .expect("error while writing");
                        // stream auto closes
                    } else {
                        self.clients[index] = Some(ChessClient::new(stream, ChessPieceColor::Black));
                    }
                },
                Err(error) => println!("tcp accept error: {error}")
            }
        }
    }
}