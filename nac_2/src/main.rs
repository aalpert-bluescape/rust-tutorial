use slint;

slint::slint! {
    GameTile := Rectangle {
        width: 64px;
        height: 64px;
        background: lightsteelblue;
        Image {
            source: @image-url("assets/X.png");
            width: parent.width;
            height: parent.height;
        }
    }

    MainWindow := Window {
        property <[int]> tile_data: [
            1,2,3,4,5,6,7,8,9
        ];

        for tile[i] in tile_data : GameTile {
            x: mod(i, 3) * 74px;
            y: floor(i / 3) * 74px;
        }
    }
}

fn main() {
    let main_window = MainWindow::new();
    //let tile_data = vec![0,0,0,0,0,0,0,0,0];
    //main_window.set_tile_data(tile_data.into())
    main_window.run();
}

