fn main() {
    let mut board = Board::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    let new_move: Move = Move::new("e2e4");
    board.print();
    board.make_move(new_move);
    board.print();
}

#[derive(Copy, Clone, PartialEq)]
enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
    None
}

#[derive(Copy, Clone, PartialEq)]
enum Color {
    White,
    Black
}

#[derive(Copy, Clone)]
struct Board {
    pieces: [Piece; 64],
}

struct Coordinate {
    file: usize,
    rank: usize,
}

struct Move {
    from: Coordinate,
    to: Coordinate,
}

impl Board {
    fn piece_at(self, x: usize, y: usize) -> Piece {
        return self.pieces[x + 8*y];
    }
    fn new() -> Board {
        let output_board: Board = Board { pieces: [Piece::None; 64] };
        return output_board;
    }
    fn new_from_fen(fen: &str) -> Board {
        let mut iter: usize = 0;
        let mut output_board = Board::new();

        for rank in fen.split("/") {
            for c in rank.chars() {
                match c {
                    'P' => output_board.pieces[iter] = Piece::Pawn(Color::White),
                    'N' => output_board.pieces[iter] = Piece::Knight(Color::White),
                    'B' => output_board.pieces[iter] = Piece::Bishop(Color::White),
                    'R' => output_board.pieces[iter] = Piece::Rook(Color::White),
                    'Q' => output_board.pieces[iter] = Piece::Queen(Color::White),
                    'K' => output_board.pieces[iter] = Piece::King(Color::White),

                    'p' => output_board.pieces[iter] = Piece::Pawn(Color::Black),
                    'n' => output_board.pieces[iter] = Piece::Knight(Color::Black),
                    'b' => output_board.pieces[iter] = Piece::Bishop(Color::Black),
                    'r' => output_board.pieces[iter] = Piece::Rook(Color::Black),
                    'q' => output_board.pieces[iter] = Piece::Queen(Color::Black),
                    'k' => output_board.pieces[iter] = Piece::King(Color::Black),

                    '8' => for _ in 0..7 {output_board.pieces[iter] = Piece::None; iter += 1;},
                    '7' => for _ in 0..6 {output_board.pieces[iter] = Piece::None; iter += 1;},
                    '6' => for _ in 0..5 {output_board.pieces[iter] = Piece::None; iter += 1;},
                    '5' => for _ in 0..4 {output_board.pieces[iter] = Piece::None; iter += 1;},
                    '4' => for _ in 0..3 {output_board.pieces[iter] = Piece::None; iter += 1;},
                    '3' => for _ in 0..2 {output_board.pieces[iter] = Piece::None; iter += 1;},
                    '2' => for _ in 0..1 {output_board.pieces[iter] = Piece::None; iter += 1;},
                    '1' => output_board.pieces[iter] = Piece::None,

                    _ => break
                }
                output_board.pieces[iter].print();
                print!(" {} ", iter);
                println!("");
                iter += 1;
            }
        }

        return output_board;
    }
    fn print(self) {
        for x in 0..8 {
            println!("");
            for y in 0..8 {
                self.pieces[8*x + y].print();
            }
        }
        println!("");
    }
    fn make_move(&mut self, var_move: Move) {
        self.pieces[var_move.to.file + 8*var_move.to.rank] = self.pieces[var_move.from.file + 8*var_move.from.rank];
        self.pieces[var_move.from.file + 8*var_move.from.rank] = Piece::None;
    }
}

impl Piece {
    fn print(self) {
        match self {
            Piece::None => print!(". "),

            Piece::Pawn(Color::White) => print!("P "),
            Piece::Knight(Color::White) => print!("N "),
            Piece::Bishop(Color::White) => print!("B "),
            Piece::Rook(Color::White) => print!("R "),
            Piece::Queen(Color::White) => print!("Q "),
            Piece::King(Color::White) => print!("K "),

            Piece::Pawn(Color::Black) => print!("p "),
            Piece::Knight(Color::Black) => print!("n "),
            Piece::Bishop(Color::Black) => print!("b "),
            Piece::Rook(Color::Black) => print!("r "),
            Piece::Queen(Color::Black) => print!("q "),
            Piece::King(Color::Black) => print!("k "),
        }
    }
}

impl Coordinate {
    fn new(uci: &str) -> Coordinate {
        let mut output_coordinate: Coordinate = Coordinate { file: 0, rank: 0 };

        match uci[..1].to_uppercase().as_str() {
            "A" => output_coordinate.file = 0,
            "B" => output_coordinate.file = 1,
            "C" => output_coordinate.file = 2,
            "D" => output_coordinate.file = 3,
            "E" => output_coordinate.file = 4,
            "F" => output_coordinate.file = 5,
            "G" => output_coordinate.file = 6,
            "H" => output_coordinate.file = 7,
            "X" => output_coordinate.file = 99,
            _ => panic!("Not a UCI square!"),
        }
        match uci[1..2].to_uppercase().as_str() {
            "1" => output_coordinate.rank = 7,
            "2" => output_coordinate.rank = 6,
            "3" => output_coordinate.rank = 5,
            "4" => output_coordinate.rank = 4,
            "5" => output_coordinate.rank = 3,
            "6" => output_coordinate.rank = 2,
            "7" => output_coordinate.rank = 1,
            "8" => output_coordinate.rank = 0,
            "X" => output_coordinate.rank = 99,
            _ => panic!("Not a UCI square!"),
        }
        return output_coordinate;
    }
}

impl Move {
    fn new(uci: &str) -> Move {
        let mut output_move: Move = Move {from: Coordinate::new("xx"), to: Coordinate::new("xx")};

        let from_coordinate: Coordinate = Coordinate::new(&uci[..2]);
        let to_coordinate: Coordinate = Coordinate::new(&uci[2..4]);

        output_move.from = from_coordinate;
        output_move.to = to_coordinate;

        return output_move;
    }
}