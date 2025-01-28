use std::net::{ TcpListener, TcpStream, Shutdown };
use std::io::{ Read, Write };

use chess_engine::{ ChessMove, ChessBoard, ChessPieceColor };

#[derive(Copy, Clone, Debug)]
pub struct ChessPacket {
    client_id: u8, // id of the client playing
    data: [u8; 2], // move is a u16 but sockets send u8
    turn: u8,      // turn number
}

#[derive(Copy, Clone, Debug)]
pub struct ChessClient {
    board: ChessBoard,
    client_id: u8,
}

pub struct ChessServer {
    clients: [Option<ChessClient>; 8], // only 8 clients at a time
    service: TcpListener,
}

impl ChessPacket {
    pub fn new(client_id: u8, data: ChessMove, turn: u8) -> Self {
        Self { client_id, data: [(data.0 & 0xff) as u8, (data.0 >> 8) as u8], turn }
    }

    pub fn from_stream(stream: &mut TcpStream) -> Self {
        let mut buffer: [u8; 4] = [0; 4];
        
        match stream.read(&mut buffer) {
            Ok(bytes) => println!("{:#?}, {}", buffer, bytes),
            Err(_) => println!("error!")
        };

        return Self { 
            client_id: buffer[0],
            data: buffer[1..3].try_into().unwrap(), 
            turn: buffer[3]
        }
    }

    pub fn into_buffer(self) -> [u8; 4] {
        [self.client_id, self.data[0], self.data[1], self.turn]
    }
}

impl ChessClient {
    pub fn new(client_id: u8, color: ChessPieceColor) -> Self {
        Self { client_id, board: ChessBoard::new(color) }
    }
}

impl ChessServer {
    pub fn new(host: &str, port: u16) -> Result<Self, String> {
        match TcpListener::bind(format!("{}:{}", host, port)) {
            Ok(service) => Ok( Self { clients: [None; 8], service }),
            Err(error) => Err(error.to_string())
        }
    }

    pub fn serve_chess_games(&mut self) {
        for stream in self.service.incoming() {
            match stream {
                Ok(mut stream) => {
                    let packet: ChessPacket = ChessPacket::from_stream(&mut stream);
                    println!("{:?}", packet);

                    stream.write("fuck".as_bytes());
                    stream.shutdown(Shutdown::Both);
                },
                Err(error) => println!("tcp accept error: {error}")
            }
        }
    }
}