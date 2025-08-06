//! Common immutable utilites that can be used through all ui code.
//!
//! Mutable utilities should be placed in the [`crate::Ui`]'s [`GlobalData`][`crate::screens::GlobalData`].

use macroquad::{
    Error,
    audio::Sound,
    prelude::{
        ImageFormat,
        coroutines::{Coroutine, start_coroutine},
    },
    text::Font,
    texture::Texture2D,
};
use std::{
    collections::HashMap,
    sync::{LazyLock, OnceLock},
};

pub fn poll_assets() {
    move_sound();
}

/// Get the sound for moves and rotates.
///
/// Uses a static [`OnceLock`] to prevent reinstantiation.
pub fn move_sound() -> Option<&'static Sound> {
    #[derive(Debug)]
    enum State {
        Waiting(Coroutine<Result<Sound, Error>>),
        Finished(Sound),
    }

    impl State {
        /// Get the sound from a finished state, panicking if not finished.
        pub fn finished_sound(&self) -> &Sound {
            match self {
                State::Waiting(_) => panic!("invariant"),
                State::Finished(sound) => &sound,
            }
        }
    }

    static SOUND: OnceLock<State> = OnceLock::new();

    match OnceLock::get(&SOUND) {
        None => {
            SOUND
                .set(State::Waiting(start_coroutine(
                    macroquad::audio::load_sound_from_bytes(include_bytes!(
                        "../assets/sound/move.wav"
                    )),
                )))
                .expect("cell should not be already initialized");
            None
        }
        Some(State::Waiting(coroutine)) => coroutine.retrieve().map(|retrieved| {
            let sound = retrieved.expect("hardcoded sound should parse correctly");
            SOUND.set(State::Finished(sound));
            SOUND
                .get()
                .expect("we just set the sound.")
                .finished_sound()
        }),
        Some(State::Finished(sound)) => Some(&sound),
    }
}

/// Get the font we use for the game.
///
/// Uses a static [`LazyLock`] to prevent reinstantiation.
pub fn font() -> &'static Font {
    static FONT: LazyLock<Font> = LazyLock::new(|| {
        macroquad::text::load_ttf_font_from_bytes(include_bytes!("../assets/OpenSans.ttf")).unwrap()
    });
    &FONT
}

/// Get an image texture from its name.
///
/// Images come from `../assets/pieces_png/` and are stored in a static [`LazyLock`] to prevent reinstantiation.
/// They're stored and queried with a [`HashMap`].
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
