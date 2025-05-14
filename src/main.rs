use macroquad::{miniquad::conf::Icon, prelude::*};

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
fn rotchess_icon() -> Icon {
    macro_rules! icon_from_path {
        ($path:expr) => {
            Image::from_file_with_format(include_bytes!($path), Some(ImageFormat::Png))
                .unwrap()
                .bytes
                .try_into()
                .unwrap()
        };
    }

    let (small, medium, big) = (
        icon_from_path!("../assets/icon/icon_small.png"),
        icon_from_path!("../assets/icon/icon_medium.png"),
        icon_from_path!("../assets/icon/icon_large.png"),
    );

    Icon { small, medium, big }
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Rotating Chess"),
        window_height: 400,
        window_width: 600,
        icon: Some(rotchess_icon()),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
