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
}

fn calculate_state(_grid: &Grid) -> GameState {
    return GameState::Open;
}

fn main() {
    let grid = Grid::new();
    println!("{:?} -> {:?}", grid, calculate_state(&grid));

}
