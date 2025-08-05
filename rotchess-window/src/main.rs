use macroquad::prelude::*;

use rotchess_window::window_conf::window_conf;
use rotchess_ui::Ui;

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = Ui::new();

    loop {
        app.update();
        app.draw();

        next_frame().await
    }
}
