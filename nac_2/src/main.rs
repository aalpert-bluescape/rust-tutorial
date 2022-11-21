use slint;

slint::slint! {
    GameTile := Rectangle {
        callback clicked;
        property <int> player;
        width: 64px;
        height: 64px;
        background: lightsteelblue;
        Image {
            source: player == 1  ?  @image-url("assets/X.png")
                : player == 2 ? @image-url("assets/O.png")
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
        for tile[i] in tile_data : GameTile {
            x: mod(i, 3) * 74px;
            y: floor(i / 3) * 74px;
            player: tile;
            clicked => {
                tile += 1;
            }

        }
    }
}

fn main() {
    let main_window = MainWindow::new();
    let tile_data = std::rc::Rc::new(slint::VecModel::from(vec![0,0,0,0,0,0,0,0,0]));
    main_window.set_tile_data(tile_data.into());
    main_window.run();
}

