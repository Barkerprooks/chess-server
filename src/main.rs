use chess_engine;

mod http;
use http::HttpRequest;

fn main() {

    HttpRequest::from_string("GET / HTTP/1.1\r\nHost: hello\r\nNice: meme\r\n\r\n");

}