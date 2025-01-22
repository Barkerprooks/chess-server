use std::net::{ TcpListener, TcpStream, SocketAddr, ToSocketAddrs };
use std::io::Read;

use chess_engine::{ ChessMove, ChessBoard, ChessPieceColor };

const INITIAL_CLIENTS: [Option<ChessClient>; 8] = [ None, None, None, None, None, None, None, None ];

#[derive(Debug)]
pub struct ChessPacket {
    data: [u8; 2], // move is a u16 but sockets send u8
    turn: u8 // turn number
}

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
        let service = TcpListener::bind(format!("{}:{}", host, port));
        match service {
            Ok(service) => Ok( Self { service, clients: INITIAL_CLIENTS } ),
            Err(_) => Err(format!("couldn't bind to address {host}:{port}"))
        }
    }

    pub fn client_connected(&mut self, stream: TcpStream) {
        let index = self.clients.iter()
            .enumerate()
            .find(|(_i, client)| client.is_some())
            .unwrap_or((0, &None)).0;
        
        println!("{}", index);
    
        self.clients[index] = Some(ChessClient::new(stream, ChessPieceColor::White));
    }

    pub fn serve_forever(&mut self) {
        let mut streams: Vec<TcpStream> = vec![];

        for stream in self.service.incoming() {
            match stream {
                Ok(stream) => streams.push(stream),
                Err(error) => println!("error: {error}")
            }

            println!("{:#?}", streams);
        }
    }
}