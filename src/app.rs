use macroquad::prelude::*;

use crate::chess::{RotchessEmulator, piece::Pieces};

const DARK_TILE_COLOR: Color = Color::from_rgba(181, 136, 99, 255);
const BACKGROUND_COLOR: Color = Color::from_rgba(240, 217, 181, 255);

pub struct App {
    chess: RotchessEmulator,
    runit_to_world_multiplier: f32,
}

impl App {
    pub fn new() -> Self {
        Self {
            chess: RotchessEmulator::with(Pieces::standard_board()),
            runit_to_world_multiplier: 0.,
        }
    }

    fn update_runit_to_world_multiplier(&mut self) {
        println!("{} {}", screen_width(), screen_height());
        self.runit_to_world_multiplier = f32::min(screen_width(), screen_height()) / 8.;
    }

    /// Converts from a rotchess unit to world unit (pixel).
    ///
    /// Must be run after we update the ratio after any screen resize, else the value is outdated.
    fn cnv(&self, a: f32) -> f32 {
        self.runit_to_world_multiplier * a
    }

    pub fn update(&mut self) {
        self.update_runit_to_world_multiplier();
        // egui_macroquad::ui(|ctx| {
        //     // egui::Window::new("My Window")
        //     //     .resizable(true)
        //     //     .show(ctx, |ui| {
        //     //         ui.label("Hello World!");
        //     //     });
        // });
    }

    fn draw_board(&self) {
        let mut top = 0;
        let mut left = 1;
        let mut next_row_immediate_dark = true;

        const NUM_TILES: u8 = 8 * 8;
        const NUM_DARK_TILES: u8 = NUM_TILES / 2;

        for _ in 0..NUM_DARK_TILES {
            draw_rectangle(
                self.cnv(left as f32),
                self.cnv(top as f32),
                self.cnv(1.),
                self.cnv(1.),
                DARK_TILE_COLOR,
            );

            left += 2;
            if left >= 8 {
                left = if next_row_immediate_dark { 0 } else { 1 };
                next_row_immediate_dark = !next_row_immediate_dark;
                top += 1;
            }
        }
    }

    pub fn draw(&self) {
        clear_background(BACKGROUND_COLOR);
        self.draw_board();

        // egui_macroquad::draw();
        // self.draw_primitives();
    }

    fn draw_primitives(&self) {
        draw_line(0., 0., 100., 100., 0.05, BLUE);
        draw_rectangle(-0.1, 110.1, 100.2, 100.2, GREEN);
        draw_circle(510., 0., 110.1, YELLOW);
    }
}
