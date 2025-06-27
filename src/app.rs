use std::{collections::HashMap, f32::consts::PI};

use macroquad::prelude::*;

use crate::chess::{
    RotchessEmulator,
    piece::{PIECE_RADIUS, Pieces},
};

const DARK_TILE_COLOR: Color = Color::from_rgba(181, 136, 99, 255);
const BACKGROUND_COLOR: Color = Color::from_rgba(240, 217, 181, 255);

/// yellowish
const SELECTED_PIECE_COLOR: Color = Color::from_rgba(255, 255, 153, 255);
/// cyanish
const MOVE_POINT_COLOR: Color = Color::from_rgba(173, 255, 244, 255);
/// red
const CAPTURE_POINT_COLOR: Color = Color::from_rgba(255, 0, 0, 255);
/// springgreen
const HITCIRCLE_COLOR: Color = Color::from_rgba(0, 255, 127, 255);

pub struct App {
    chess: RotchessEmulator,
    runit_to_world_multiplier: f32,
    images: HashMap<String, Texture2D>,
}

impl App {
    fn generate_images() -> HashMap<String, Texture2D> {
        let mut images = HashMap::new();
        images.insert(
            "piece_bishopB1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_bishopB1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_bishopW1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_bishopW1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_kingB1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_kingB1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_kingW1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_kingW1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_knightB1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_knightB1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_knightW1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_knightW1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_pawnB1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_pawnB1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_pawnW1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_pawnW1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_queenB1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_queenB1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_queenW1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_queenW1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_rookB1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_rookB1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_rookW1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_rookW1.png"),
                Some(ImageFormat::Png),
            ),
        );

        images
    }

    pub fn new() -> Self {
        Self {
            chess: RotchessEmulator::with(Pieces::standard_board()),
            runit_to_world_multiplier: 0.,
            images: App::generate_images(),
        }
    }

    fn update_runit_to_world_multiplier(&mut self) {
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
        self.chess.kaboom();
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

    fn draw_pieces(&self) {
        /// Size as fraction of 1.
        const PIECE_SIZE: f32 = 0.9;
        for piece in self.chess.pieces() {
            draw_texture_ex(
                &self
                    .images
                    .get(&format!(
                        "piece_{}{}1",
                        piece.kind().to_file_desc(),
                        piece.side().to_file_desc()
                    ))
                    .expect("Pieces should correctly map to the file descrs."),
                self.cnv(piece.x() - PIECE_SIZE / 2.),
                self.cnv(piece.y() - PIECE_SIZE / 2.),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2 {
                        x: self.cnv(PIECE_SIZE),
                        y: self.cnv(PIECE_SIZE),
                    }),
                    rotation: piece.angle() - PI / 2.,
                    ..Default::default()
                },
            );
            // draw_circle_lines(
            //     self.cnv(piece.x()),
            //     self.cnv(piece.y()),
            //     self.cnv(PIECE_RADIUS),
            //     1.,
            //     HITCIRCLE_COLOR,
            // );
        }
    }

    pub fn draw(&self) {
        clear_background(BACKGROUND_COLOR);
        self.draw_board();

        // egui_macroquad::draw();
        self.draw_pieces();
    }
}
