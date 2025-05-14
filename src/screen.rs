use macroquad::color::Color;
use macroquad::input;
use macroquad::input::MouseButton;
use statig::Response;
use statig::state_machine;

pub struct GlobalData {
    pub bg_color: Color,
}

/// Dummy value enum to satisfy type checking. If we leave the start state without an event param,
/// statig miiiight let us use &() in handle_with_context. This is just here because
/// we're probably hand-coding an event queue later anyways.
pub enum Event {
    /// Dummy value. will be changed.
    Dummy1,
    /// Dummy value. will be changed.
    Dummy2,
}

#[derive(Default)]
pub struct Screen;

#[state_machine(initial = "State::start()")]
impl Screen {
    #[state]
    fn start(event: &Event, context: &mut GlobalData) -> Response<State> {
        if input::is_mouse_button_pressed(MouseButton::Left) {
            context.bg_color = Color {
                r: rand::random_range(0.0..1.0),
                g: rand::random_range(0.0..1.0),
                b: rand::random_range(0.0..1.0),
                a: 1.0,
            }
        }
        Response::Super
    }
}
