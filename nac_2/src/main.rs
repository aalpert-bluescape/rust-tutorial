fn main() {
    MainWindow::new().run();
}

slint::slint! {
    MainWindow := Window {
        Text {
            text: "A";
        }
    }
}
