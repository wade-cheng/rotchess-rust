use macroquad::color::Color;
use macroquad::color::colors::*;
use macroquad::input;
use macroquad::input::KeyCode;
use macroquad::input::MouseButton;
use macroquad::shapes;
use macroquad::text;
use macroquad::window;
use statig::Response;
use statig::state_machine;

pub enum Command {
    Update,
    Render,
}

pub struct GlobalData {
    pub tick_command: Option<Command>,
    pub bg_color: Color,
}

impl GlobalData {
    pub fn new() -> Self {
        Self {
            tick_command: None,
            bg_color: macroquad::color::RED,
        }
    }
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
        match context.tick_command {
            Some(Command::Update) => Screen::start_update(event, context),
            Some(Command::Render) => Screen::start_render(event, context),
            None => unreachable!(),
        }
    }

    fn start_update(event: &Event, context: &mut GlobalData) -> Response<State> {
        if input::is_mouse_button_pressed(MouseButton::Left) {
            context.bg_color = Color {
                r: rand::random_range(0.0..1.0),
                g: rand::random_range(0.0..1.0),
                b: rand::random_range(0.0..1.0),
                a: 1.0,
            }
        }

        if input::is_mouse_button_pressed(MouseButton::Right) {
            return Response::Transition(State::darkness());
        }

        Response::Handled
    }

    fn start_render(event: &Event, context: &GlobalData) -> Response<State> {
        window::clear_background(context.bg_color);

        shapes::draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        shapes::draw_rectangle(
            window::screen_width() / 2.0 - 60.0,
            100.0,
            120.0,
            60.0,
            GREEN,
        );
        let (x, y) = input::mouse_position();
        shapes::draw_circle(x, y, 15.0, YELLOW);
        text::draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        Response::Handled
    }

    #[state]
    fn darkness(event: &Event, context: &mut GlobalData) -> Response<State> {
        match context.tick_command {
            Some(Command::Update) => Screen::darkness_update(event, context),
            Some(Command::Render) => Screen::darkness_render(event, context),
            None => unreachable!(),
        }
    }

    fn darkness_update(event: &Event, context: &mut GlobalData) -> Response<State> {
        if input::is_key_pressed(KeyCode::Space) {
            context.bg_color = Color {
                r: rand::random_range(0.0..0.3),
                g: rand::random_range(0.0..0.3),
                b: rand::random_range(0.0..0.3),
                a: 1.0,
            }
        }

        if input::is_key_pressed(KeyCode::A) {
            return Response::Transition(State::start());
        }

        Response::Handled
    }

    fn darkness_render(event: &Event, context: &GlobalData) -> Response<State> {
        window::clear_background(context.bg_color);

        let (x, y) = input::mouse_position();
        shapes::draw_circle(x, y, 15.0, BLACK);
        text::draw_text("HELLO", 20.0, 20.0, 20.0, WHITE);

        Response::Handled
    }
}
