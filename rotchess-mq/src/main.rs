use macroquad::prelude::*;

use rotchess_mq::app::App;
use rotchess_mq::common::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new();

    loop {
        app.update();
        app.draw();

        next_frame().await
    }
}
