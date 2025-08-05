use macroquad::miniquad::conf::Icon;
use macroquad::prelude::ImageFormat;
use macroquad::texture::Image;
use macroquad::window::Conf;

/// Generate the icon for the game.
///
/// Implementation uses a macro because reading an image via macroquad requires a macro, so unfortunately something like
///
/// ```ignore
/// fn rotchess_icon() -> Icon {
///     let [small, medium, big] = [
///         "../assets/icon/icon_small.png",
///         "../assets/icon/icon_medium.png",
///         "../assets/icon/icon_large.png",
///     ]
///     .map(|img| {
///         Image::from_file_with_format(include_bytes!(img), Some(ImageFormat::Png))
///             .unwrap()
///             .bytes
///             .try_into()
///             .unwrap()
///     });
///
///     Icon { small, medium, big }
/// }
/// ```
///
/// does not work.
fn rotchess_icon() -> Icon {
    macro_rules! icon_from_path {
        ($path:expr) => {
            Image::from_file_with_format(include_bytes!($path), Some(ImageFormat::Png))
                .unwrap()
                .bytes
                .try_into()
                .expect("Image file should've been convertible to icon.")
        };
    }

    let (small, medium, big) = (
        icon_from_path!("../assets/icon/icon_small.png"),
        icon_from_path!("../assets/icon/icon_medium.png"),
        icon_from_path!("../assets/icon/icon_large.png"),
    );

    Icon { small, medium, big }
}

/// Generate the window config for the game.
///
/// This should only be used literally once, so we don't bother `LazyLock`ing it under the hood.
pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Rotating Chess"),
        window_height: 400,
        window_width: 400,
        icon: Some(rotchess_icon()),
        sample_count: 0, // remove antialiasing
        high_dpi: true,
        ..Default::default()
    }
}
