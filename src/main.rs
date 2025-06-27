use macroquad::prelude::*;

use rotchess_mq::app::App;
use rotchess_mq::icon;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Rotating Chess"),
        window_height: 400,
        window_width: 600,
        icon: Some(icon::rotchess_icon()),
        sample_count: 0, // remove antialiasing
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new();

    loop {
        app.update();
        app.draw();

        next_frame().await
    }
}
