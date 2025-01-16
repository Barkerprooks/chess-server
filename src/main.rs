pub use chess_engine;

fn main() {

    let color = chess_engine::ChessPieceColor::White;
    let board = chess_engine::ChessBoard::new(color);

    let piece = board.tile(&chess_engine::V2 { x: 0, y: 0 }).piece();

    println!("piece at 0, 0: {:?}", piece.unwrap());
}
