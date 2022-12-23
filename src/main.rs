fn main() {
    let mut board = fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w HAkq - 0 1");
    print_board(board);
    move_from_uci(&mut board, "e2e4");
    print_board(board);
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Piece {
    Pawn, Knight, Bishop, Rook, Queen, King
}

#[derive(Copy, Clone)]
enum Color {
    White, Black
}

type Board = [(Option<Piece>, Option<Color>); 64];

fn print_piece(piece: Piece, color: Color) {
    match (piece, color) {

        (Piece::Pawn, Color::White) => print!("P "),
        (Piece::Knight, Color::White) => print!("N "),
        (Piece::Bishop, Color::White) => print!("B "),
        (Piece::Rook, Color::White) => print!("R "),
        (Piece::Queen, Color::White) => print!("Q "),
        (Piece::King, Color::White) => print!("K "),

        (Piece::Pawn, Color::Black) => print!("p "),
        (Piece::Knight, Color::Black) => print!("n "),
        (Piece::Bishop, Color::Black) => print!("b "),
        (Piece::Rook, Color::Black) => print!("r "),
        (Piece::Queen, Color::Black) => print!("q "),
        (Piece::King, Color::Black) => print!("k "),
    }
}

impl Piece {
    fn generate_moves(self, iter:usize, board: &mut Board) -> Vec<usize> {
        match self {
            Piece::Pawn => vec![0],
            Piece::Bishop => vec![0],
            Piece::Knight => self.knight_squares(iter),
            Piece::Rook => vec![0],
            Piece::Queen => vec![0],
            Piece::King => self.king_squares(iter, board),
        }
    }
    fn knight_squares(self, iter: usize) -> Vec<usize> {
        let output_vector: Vec<usize> = vec![iter + 10, iter + 17, iter + 15, iter + 6, iter - 10, iter - 17, iter - 15, iter - 6];
        return output_vector;
    }
    fn king_squares(self, iter: usize, board: &mut Board) -> Vec<usize> {

        let mut output_vector: Vec<usize> = vec![];
        // manually adding all squares, because the king does not have many. This doesn't include castling.
        if board[iter - 9].0 == None {output_vector.push(iter - 9);}
        if board[iter - 8].0 == None {output_vector.push(iter - 8);}
        if board[iter - 7].0 == None {output_vector.push(iter - 7);}
        if board[iter - 1].0 == None {output_vector.push(iter - 1);}
        if board[iter + 1].0 == None {output_vector.push(iter + 1);}
        if board[iter + 7].0 == None {output_vector.push(iter + 7);}
        if board[iter + 8].0 == None {output_vector.push(iter + 8);}
        if board[iter + 9].0 == None {output_vector.push(iter + 9);}

        return output_vector;
    }
}

fn fen_to_board(fen: &str) -> Board {
    let mut iter: usize = 0;
    let mut board: Board = [(None, None); 64];

    for rank in fen.split("/") {
        for c in rank.chars() {
            match c {
                'P' => board[iter] = (Some(Piece::Pawn), Some(Color::White)),
                'N' => board[iter] = (Some(Piece::Knight), Some(Color::White)),
                'B' => board[iter] = (Some(Piece::Bishop), Some(Color::White)),
                'R' => board[iter] = (Some(Piece::Rook), Some(Color::White)),
                'Q' => board[iter] = (Some(Piece::Queen), Some(Color::White)),
                'K' => board[iter] = (Some(Piece::King), Some(Color::White)),

                'p' => board[iter] = (Some(Piece::Pawn), Some(Color::Black)),
                'n' => board[iter] = (Some(Piece::Knight), Some(Color::Black)),
                'b' => board[iter] = (Some(Piece::Bishop), Some(Color::Black)),
                'r' => board[iter] = (Some(Piece::Rook), Some(Color::Black)),
                'q' => board[iter] = (Some(Piece::Queen), Some(Color::Black)),
                'k' => board[iter] = (Some(Piece::King), Some(Color::Black)),

                '8' => for _ in 0..7 {board[iter] = (None, None); iter += 1;},
                '7' => for _ in 0..6 {board[iter] = (None, None); iter += 1;},
                '6' => for _ in 0..5 {board[iter] = (None, None); iter += 1;},
                '5' => for _ in 0..4 {board[iter] = (None, None); iter += 1;},
                '4' => for _ in 0..3 {board[iter] = (None, None); iter += 1;},
                '3' => for _ in 0..2 {board[iter] = (None, None); iter += 1;},
                '2' => for _ in 0..1 {board[iter] = (None, None); iter += 1;},
                '1' => board[iter] = (None, None),

                _ => break
            }
            iter += 1;
        }}
    return board;
}

fn print_board(board: Board) {
    for x in 0..8 {
        println!("");
        for y in 0..8 {
            let iter = 8*x + y;
            if board[iter].0 == None {print!(". ")}
            else {print_piece(board[iter].0.unwrap(), board[iter].1.unwrap());}
        }
    }
    println!("");
}

fn square_to_iter(square: &str) -> usize {
    let square = square.to_string().to_uppercase();
    let rank: usize = square[1..2].parse::<usize>().unwrap();
    let mut output: usize = 8 * (8 - rank);

    match &square[..1] {
        "A" => output += 0,
        "B" => output += 1,
        "C" => output += 2,
        "D" => output += 3,
        "E" => output += 4,
        "F" => output += 5,
        "G" => output += 6,
        "H" => output += 7,
        &_ => (),
    }
    return output;
}

fn move_from_uci(board: &mut Board, uci: &str) {
    let square_from = square_to_iter(&uci[..2]);
    let square_to = square_to_iter(&uci[2..4]);
    let piece_from: Piece;

    if board[square_from].0 != None {piece_from = board[square_from].0.unwrap();}
    else {panic!("No piece at {}", square_from)}

    for x in 0..piece_from.generate_moves(square_from, board).len() {
        if square_to == piece_from.generate_moves(square_from, board)[x] {
            board[square_to] = board[square_from];
            board[square_from] = (None, None);
            break;
        }
    }
}