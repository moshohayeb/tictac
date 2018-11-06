use std::io::{self, Write};

// Questions
// 1. How to implement Display for Tile (i.e. Option<Piece>) instead of for the Render trait
//    or a standalone function that does the translation

type Board = [[Tile; 3]; 3];
type Tile = Option<Piece>;

 pub struct Game {
    board: Board,
    turn: Piece,
    winner: Option<Piece>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Piece {
    X,
    O,
}

impl std::ops::Not for Piece {
    type Output = Piece;
    fn not(self) -> Piece {
        match self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}

#[derive(Debug)]
pub enum GameError {
    OutOfRange,
    NotEmpty,
}

trait Render {
    fn render(&self) -> &'static str;
}

impl Render for Tile {
    fn render(&self) -> &'static str {
        match self {
            Option::Some(v) if v == &Piece::X => "X",
            Option::Some(v) if v == &Piece::O => "O",
            Option::None => ".",
            _ => "!",
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Default::default(),
            turn: Piece::X,
            winner: None,
        }
    }

    fn read_next_move(&self) -> (usize, usize) {
        let mut buf = String::new();
        loop {
            buf.clear();

            print!("Reading move for piece '{:?}': ", self.turn);
            let _ = io::stdout().flush();

            let s = match io::stdin().read_line(&mut buf) {
                Ok(_) => buf.trim_right(),
                Err(why) => {
                    eprintln!("io error: {}", why);
                    continue;
                }
            };

            if s.chars().count() != 2 {
                eprintln!("move format is columnrow (e.g. a2)");
                continue;
            }

            // We've already verified the len
            let col = s.chars().nth(0).unwrap();
            let row = s.chars().nth(1).unwrap();

            let col = match col {
                'A' | 'a' => 0,
                'B' | 'b' => 1,
                'C' | 'c' => 2,
                _ => {
                    println!("bad column {}", col);
                    continue;
                }
            };

            let row = match row.to_digit(10) {
                Some(v) if v <= 2 => v,
                _ => {
                    println!("bad row {}", row);
                    continue;
                }
            };

            return (row as usize, col as usize);
        }
    }

    fn draw(&self) {
        println!("");
        println!("+++++++++ Board +++++++++");
        println!("-------------------------");
        println!("\tA\tB\tC");
        println!("-------------------------");
        for row in 0..=2 {
            print!("{}|\t", row);
            for col in 0..=2 {
                print!("{}\t", self.board[row][col].render());
            }
            println!("");
        }
        println!("+++++++++++++++++++++++++");
        println!("");
    }

    fn update(&mut self) {
        let board = &self.board;

        // diagonal
        let piece = board[1][1];
        if (piece == board[0][0] && piece == board[2][2])
            || (piece == board[0][2] && piece == board[2][0])
        {
            self.winner = piece;
            return;
        }

        // horizontal
        for i in 0..=2 {
            let piece = board[i][0];
            if piece == board[i][1] && piece == board[i][2] {
                self.winner = piece;
                return;
            }
        }

        // vertical
        for i in 0..=2 {
            let piece = board[0][i];
            if piece == board[1][i] && piece == board[2][i] {
                self.winner = piece;
                return;
            }
        }
    }

    fn play(&mut self, piece: Piece, row: usize, col: usize) -> Result<(), GameError> {
        if let Some(_) = self.board[row][col] {
            return Err(GameError::NotEmpty);
        }

        if row > 2 || col > 2 {
            return Err(GameError::OutOfRange);
        }

        self.board[row][col] = Some(piece);
        self.turn = !piece;
        Ok(())
    }

    fn is_done(&self) -> bool {
        if self.winner != None {
            return true;
        }

        // see if all slots are occupied
        self.board.iter().flatten().all(|v| {
            *v != None
        })
    }

    pub fn start(&mut self) {
        while !self.is_done() {
            self.draw();
            let (col, row) = self.read_next_move();
            if let Err(why) = self.play(self.turn, col, row) {
                println!("err: {:?}...", why);
            }
            self.update();
        }

        println!("");
        let result = match self.winner {
            None => "Draw",
            Some(Piece::X) => "Winner: X",
            Some(Piece::O) => "Winner: O",
        };
        println!("=========================");
        println!("   Game Over: {} ", result);
        println!("=========================");
        self.draw();
    }
}
