use macroquad::prelude::Conf; // from miniquad
use macroquad::window;

use rotchess_mq::App;
use rotchess_mq::icon;
use rotchess_mq::logic::event_queue;
use rotchess_mq::logic::screen_state::Event;

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
        let mut events: Vec<Event> = Vec::new();
        event_queue::refill_event_queue(&mut events);
        app.update(events);
        app.render();
        window::next_frame().await
    }
}
