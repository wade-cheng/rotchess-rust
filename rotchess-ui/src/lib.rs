//! User interface code for all the non-chessboard stuff.

use macroquad::text::Font;
use std::sync::LazyLock;

mod screens;
use screens::*;

pub fn font() -> &'static Font {
    static FONT: LazyLock<Font> = LazyLock::new(|| {
        macroquad::text::load_ttf_font_from_bytes(include_bytes!("../assets/OpenSans.ttf")).unwrap()
    });
    &FONT
}

pub struct Ui {
    curr_screen_id: ScreenId,
    global_data: GlobalData,
    screen_pool: [Box<dyn Screen>; 6],
}

impl Default for Ui {
    fn default() -> Self {
        let curr_screen_id = ScreenId::Splash;
        let global_data = Default::default();
        let screen_pool = [
            Box::new(Game::new()) as Box<dyn Screen>,
            Box::new(Splash::new()) as Box<dyn Screen>,
            Box::new(Singleplayer::new()) as Box<dyn Screen>,
            Box::new(Multiplayer::new()) as Box<dyn Screen>,
            Box::new(Load::new()) as Box<dyn Screen>,
            Box::new(Settings::new()) as Box<dyn Screen>,
        ];

        for (i, screen) in screen_pool.iter().enumerate() {
            // we check this at runtime.
            // i'm sure there's ways to get the end behavior with macros,
            // either written myself or with enum ergonomics crates, but ew compile times.
            assert_eq!(i, screen.id().pool_idx(), "pool indicies must be correct.")
        }

        Self {
            curr_screen_id,
            global_data,
            screen_pool,
        }
    }
}

impl Ui {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self) {
        let curr_screen_id = &mut self.curr_screen_id;
        let global_data = &mut self.global_data;
        let screen_pool = &mut self.screen_pool;

        let curr_screen = &mut screen_pool[curr_screen_id.pool_idx()];
        let screen_change = curr_screen.update(global_data);

        if let Some(id) = screen_change {
            curr_screen.exit(global_data);

            self.curr_screen_id = id;

            let new_screen = &mut screen_pool[id.pool_idx()];
            new_screen.enter(global_data);
        }
    }

    pub fn draw(&self) {
        self.screen_pool[self.curr_screen_id.pool_idx()].draw();
    }
}
