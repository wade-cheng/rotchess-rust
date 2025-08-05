//! An app that lets users play and see (update/draw) chess, computed with help from [`rotchess_core`] and macroquad.

use rotchess_ui::Ui;

pub struct App {
    ui: Ui,
}

impl App {
    pub fn new() -> Self {
        Self { ui: Ui::new() }
    }

    pub fn update(&mut self) {
        self.ui.update();
    }

    pub fn draw(&self) {
        self.ui.draw();
    }
}
