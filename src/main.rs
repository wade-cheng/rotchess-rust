use macroquad::prelude::Conf; // from miniquad
use macroquad::window;
use rotchess_mq::App;
use rotchess_mq::event_queue;
use rotchess_mq::icon;

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
        let events = event_queue::get_event_queue();
        app.update();
        app.render();
        window::next_frame().await
    }
}
