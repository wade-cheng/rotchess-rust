mod screen;
use screen::{Event, GlobalData, Screen};

use statig::blocking::{IntoStateMachineExt, StateMachine};

pub struct App {
    context: GlobalData,
    screen: StateMachine<Screen>,
}

impl App {
    pub fn new() -> Self {
        App {
            context: GlobalData::new(),
            screen: Screen::default().state_machine(),
        }
    }

    pub fn update(&mut self) {
        self.screen
            .handle_with_context(&Event::Dummy1, &mut self.context);
    }

    pub fn render(&mut self) {
        self.screen
            .handle_with_context(&Event::Dummy1, &mut self.context);
    }
}
