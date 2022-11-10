use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum GameError {
    Parse(ParseError),
    InvalidMove(InvalidMoveError),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::Parse(e) => write!(f, "{}", e),
            GameError::InvalidMove(e) => write!(f, "{}", e),
        }
    }
}


#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot parse command.")
    }
}

#[derive(Debug, Clone)]
struct InvalidMoveError;
impl fmt::Display for InvalidMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Move is invalid.")
    }
}

#[derive(Debug, Clone)]
enum Player {
    X,
    O
}

fn spot_char(o: &Option<Player>) -> String {
    match o {
        Some(Player::X) => "X".to_owned(),
        Some(Player::O) => "O".to_owned(),
        None => " ".to_owned(),
    }
}

#[derive(Debug, Clone)]
struct Board {
    winner: Option<Player>,
    board: [[Option<Player>; 3]; 3], // 2D 3x3 array
}

impl Board {
    fn new() -> Board {
        return Board {
            winner: None,
            board: [[None, None, None], [None, None, None], [None, None, None]],
        }
    }

    fn print(&self) {
        for i in 0..3 {
            println!("{}|{}|{}",
                spot_char(&self.board[i][0]),
                spot_char(&self.board[i][1]),
                spot_char(&self.board[i][2]),
                );
            if i != 2 {
                println!("-----");
            } else {
                println!("");
            }
        }
    }

    fn play(&mut self, cmd: &str, player: Player) -> Result<(), GameError> {
        let parts: Vec<Result<usize, std::num::ParseIntError>> = cmd.split(',').map(usize::from_str).collect();
        if parts.len() != 2 || parts[0].is_err() || parts[1].is_err() {
            return Err(GameError::Parse(ParseError));
        }
        let row = parts[0].clone().unwrap() - 1;
        let col = parts[1].clone().unwrap() - 1;
        if row > 2 || col > 2 {
            return Err(GameError::InvalidMove(InvalidMoveError));
        }
        if self.board[row][col].is_some() {
            return Err(GameError::InvalidMove(InvalidMoveError));
        }
        self.board[row][col] = Some(player);
        self.check_winner();
        self.play_ai();
        self.check_winner();
        
        Ok(())
    }

    fn play_ai(&mut self) {
    }

    fn check_winner(&mut self) { // Would be better if it was not &mut and a more functional style
        let mut rows: [i32;3] = [0,0,0];
        let mut cols: [i32;3] = [0,0,0];
        let mut diag_left: i32 = 0;
        let mut diag_right: i32 = 0;
        let op_int = |op: &Option<Player>| -> i32 {
            match op {
                Some(p) => match p { 
                    Player::X => 1,
                    Player::O => -1,
                },
                None => 0,
            }
        };

        for i in 0..3 {
            for j in 0..3 {
                rows[i] += op_int(&self.board[i][j]);
                cols[j] += op_int(&self.board[i][j]);
                diag_left  += op_int(&self.board[j][i]);
                diag_right += op_int(&self.board[i][j]);
            }
        }

        let all = rows.concat(&cols).append(diag_left).append(diag_right);

    }
}

fn main() {
    let mut board = Board::new();
    let mut redraw_flag = true;
    loop {
        // Print state
        if redraw_flag {
            println!("\x1B\x63Noughts & Crosses\n"); //\x1B\x63 clears previous terminal output
            board.print();
            redraw_flag = false;
        }

        println!("Your move:");
        // Get Input
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        // Process input
        match line.trim_end() {
            "exit" => break,
            "redraw" => redraw_flag = true,
            x => {
                match board.play(x, Player::X) {
                    Ok(_) => redraw_flag = true,
                    Err(e) => println!("{}", e),
                }
            }
        }
    }
    println!("Goodbye");
}
