use slint;
use slint::Model;

slint::slint! {
    GameTile := Rectangle {
        callback clicked;
        property <int> player;
        width: 64px;
        height: 64px;
        Image {
            source: player == 1  ?  @image-url("assets/X.png")
                : player == -1 ? @image-url("assets/O.png")
                : @image-url("assets/empty.png");
            width: parent.width;
            height: parent.height;
        }

        TouchArea {
            clicked => {
                root.clicked();
            }
        }
    }

    MainWindow := Window {
        property <[int]> tile_data;
        property <bool> locked;
        property <bool> x_last;
        property <string> text: "Game on!";
        callback check_state();
        background: locked ? red : green;
        for tile[i] in tile_data : GameTile {
            x: mod(i, 3) * 74px;
            y: floor(i / 3) * 74px;
            player: tile;
            clicked => {
                if (!locked) {
                    tile = x_last ? -1 : 1;
                    x_last = !x_last;

                    root.check_state();
                }
            }
        }
        Text {
            x: 0;
            y: 3*74px + 3px;
            text: root.text;
        }
    }
}

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

fn main() {
    let main_window = MainWindow::new();
    let tile_data = std::rc::Rc::new(slint::VecModel::from(vec![0,0,0,0,0,0,0,0,0]));
    let main_window_weak = main_window.as_weak();
    main_window.set_tile_data(tile_data.clone().into());
    main_window.on_check_state(move || {
        let mut acc = 0;
        let mut grid = Grid::new();
        for k in tile_data.iter() {
            grid.board[acc%3][acc/3] = k;
            println!("{}", k);
            acc += 1;
        } 
        let state = calculate_state(&grid);
        println!("{:?} -> {:?}", grid, state);
        match state {
            GameState::Open => {
            },
            GameState::OWon=> {
                main_window_weak.unwrap().set_locked(true);
                main_window_weak.unwrap().set_text("O wins!".into());
            },
            GameState::XWon=> {
                main_window_weak.unwrap().set_locked(true);
                main_window_weak.unwrap().set_text("X wins!".into());
            },
            _ => {
                main_window_weak.unwrap().set_locked(true);
                main_window_weak.unwrap().set_text("Draw".into());
            },
        }
    });
    main_window.run();
}

