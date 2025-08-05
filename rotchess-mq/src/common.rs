use macroquad::miniquad::conf::Icon;
use macroquad::prelude::ImageFormat;
use macroquad::texture::Image;

/// Gets the icon for the game.
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
pub fn rotchess_icon() -> Icon {
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
