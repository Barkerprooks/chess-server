// Yes, I want to roll my own http server. Shut up.

use std::{collections::HashMap, io::Read, net::TcpStream};

pub enum HttpProtocol { V1_0, V1_1, V2_0 }

impl HttpProtocol {
    pub fn to_string(&self) -> &str {
        match self {
            Self::V1_0 => "HTTP/1.0",
            Self::V1_1 => "HTTP/1.1",
            Self::V2_0 => "HTTP/2.0"
        }
    }
}

pub struct HttpRequest {
    pub protocol: HttpProtocol,
    pub headers: HashMap<String, String>,
    pub verb: String,
    pub path: String,
    pub port: u16,
}

impl HttpRequest {

    // pub fn from_stream(stream: &mut TcpStream) -> Result<Self, ()> {
    //     let mut request: String = String::new();
    //     match stream.read_to_string(&mut request) {
    //         Ok(_) => Self::from_string(&request),
    //         Err(_) => Err(())
    //     }
    // }

    pub fn from_string(string: &str) {
        let segments: Vec<&str> = string.split("\r\n").collect();

        for segment in segments {
            println!("{}", segment.len());
        }
    }
}