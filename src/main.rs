use macroquad::prelude::Conf; // from miniquad
use macroquad::window;
use rotchess_mq::App;

mod icon;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Rotating Chess"),
        window_height: 400,
        window_width: 600,
        icon: Some(icon::rotchess_icon()),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new();
    loop {
        app.update();
        app.render();
        window::next_frame().await
    }
}
