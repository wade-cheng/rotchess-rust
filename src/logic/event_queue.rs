use macroquad::input;

use crate::logic::screen_state::Event;

/// Refills an event queue. The queue will be cleared.
pub fn refill_event_queue(events: &mut Vec<Event>) {
    events.clear();
    let (x, y) = input::mouse_position();

    if input::is_mouse_button_pressed(input::MouseButton::Left) {
        events.push(Event::LeftClick { x, y });
    }
    if input::is_mouse_button_pressed(input::MouseButton::Right) {
        events.push(Event::RightClick { x, y });
    }
    if input::is_key_pressed(input::KeyCode::Space) {
        events.push(Event::Space);
    }
    if input::is_key_pressed(input::KeyCode::A) {
        events.push(Event::A);
    }
}
