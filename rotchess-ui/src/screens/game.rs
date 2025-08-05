use super::{GlobalData, Screen, ScreenId};

pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for Game {
    fn id(&self) -> ScreenId {
        ScreenId::Game
    }

    fn enter(&mut self, global_data: &mut GlobalData) {}

    fn exit(&mut self, global_data: &mut GlobalData) {}

    fn update(&mut self, global_data: &mut GlobalData) -> Option<ScreenId> {
        None
    }

    fn draw(&self) {}
}
