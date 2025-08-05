use macroquad::{prelude::ImageFormat, text::Font, texture::Texture2D};
use std::{collections::HashMap, sync::LazyLock};

pub fn font() -> &'static Font {
    static FONT: LazyLock<Font> = LazyLock::new(|| {
        macroquad::text::load_ttf_font_from_bytes(include_bytes!("../assets/OpenSans.ttf")).unwrap()
    });
    &FONT
}

pub fn get_image_unchecked(name: &str) -> &Texture2D {
    macro_rules! get_piece_images {
        ( $( $x:expr ),* $(,)? ) => {
            {
                let mut images = HashMap::new();
                $(
                    images.insert(
                        $x.to_string(),
                        Texture2D::from_file_with_format(
                            include_bytes!(concat!("../assets/pieces_png/", $x, ".png")),
                            Some(ImageFormat::Png),
                        ),
                    );
                )*
                images
            }
        };
    }

    static IMAGES: LazyLock<HashMap<String, Texture2D>> = LazyLock::new(|| {
        get_piece_images!(
            "piece_bishopB1",
            "piece_bishopW1",
            "piece_kingB1",
            "piece_kingW1",
            "piece_knightB1",
            "piece_knightW1",
            "piece_pawnB1",
            "piece_pawnW1",
            "piece_queenB1",
            "piece_queenW1",
            "piece_rookB1",
            "piece_rookW1",
        )
    });
    IMAGES
        .get(name)
        .expect("queried for a nonexistent image texture.")
}
