pub mod game;
pub mod load;
pub mod lobby;
pub mod settings;
pub mod splash;

pub use game::Game;
pub use load::Load;
pub use lobby::Lobby;
pub use settings::Settings;
pub use splash::Splash;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ScreenId {
    Game,
    Splash,
    Lobby,
    Load,
    Settings,
}

impl ScreenId {
    pub fn pool_idx(&self) -> usize {
        *self as usize
    }
}

/// Data that could be used throughout any game screen.
pub struct GlobalData {}

impl Default for GlobalData {
    fn default() -> Self {
        Self {}
    }
}

pub trait Screen {
    fn id(&self) -> ScreenId;
    fn enter(&mut self, global_data: &mut GlobalData);
    fn exit(&mut self, global_data: &mut GlobalData);
    fn update(&mut self, global_data: &mut GlobalData) -> Option<ScreenId>;
    fn draw(&self);
}
