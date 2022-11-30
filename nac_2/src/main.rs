#[derive(Debug, Clone)]
struct Grid {
    board: [[i32; 3]; 3], // 2D 3x3 array
}

impl Grid {
    fn new() -> Grid {
        return Grid {
            board: [[0,0,0], [0,0,0], [0,0,0]],
        }
    }
}

#[derive(Debug, Clone)]
enum GameState {
    Open,
    XWon,
    OWon,
    Draw,
}

fn calculate_state(grid: &Grid) -> GameState
{
    let mut rows: [i32;3] = [0,0,0];
    let mut cols: [i32;3] = [0,0,0];
    let mut diag_left: i32 = 0;
    let mut diag_right: i32 = 0;
    let mut full: i32 = 0;

    for i in 0..3 {
        for j in 0..3 {
            rows[i] += grid.board[i][j];
            cols[j] += grid.board[i][j];
            if grid.board[i][j] != 0 {
                full += 1;
            }
        }
        diag_left  += grid.board[i][i];
        diag_right += grid.board[i][2-i];
    }

    let mut all = vec![];
    all.extend(rows);
    all.extend(cols);
    all.push(diag_left);
    all.push(diag_right);
    
    for v in all {
        if v == 3 {
            return GameState::XWon;
        } else if v == -3 {
            return GameState::OWon;
        }
    }

    if full == 9 {
        return GameState::Draw;
    }

    return GameState::Open;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let grid = Grid::new();
    }

    #[test]
    fn test_calculate_state() {
        let mut grid = Grid::new();
    }
}

fn main() {
    let grid = Grid::new();
    println!("{:?} -> {:?}", grid, calculate_state(&grid));

}
