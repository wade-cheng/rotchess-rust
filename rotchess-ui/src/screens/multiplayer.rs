use super::{GlobalData, Screen, ScreenId};

pub struct Multiplayer {}

impl Multiplayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for Multiplayer {
    fn id(&self) -> ScreenId {
        ScreenId::Multiplayer
    }

    fn enter(&mut self, global_data: &mut GlobalData) {}

    fn exit(&mut self, global_data: &mut GlobalData) {}

    fn update(&mut self, global_data: &mut GlobalData) -> Option<ScreenId> {
        None
    }

    fn draw(&self) {}
}
