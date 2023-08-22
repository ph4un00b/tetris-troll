use macroquad::{
    hash, logging,
    miniquad::date::now,
    prelude::{debug, load_file, vec2, RectOffset, WHITE, YELLOW},
    text::{draw_text, measure_text},
    texture::load_image,
    ui::{root_ui, Skin},
    window::{screen_height, screen_width},
};

use crate::constants::WINDOWS_SIZE;

pub struct UI;

impl UI {
    pub async fn init() {
        let window_background = load_image("assets/window_background.png").await.unwrap();
        let button_background = load_image("assets/button_background.png").await.unwrap();
        let button_clicked_background = load_image("assets/button_clicked_background.png")
            .await
            .unwrap();
        let font = load_file("assets/atari_games.ttf").await.unwrap();

        let window_style = root_ui()
            .style_builder()
            .background(window_background)
            .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
            .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(button_background)
            .background_clicked(button_clicked_background)
            .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
            .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
            .font(&font)
            .unwrap()
            .text_color(WHITE)
            .font_size(64)
            .build();

        let label_style = root_ui()
            .style_builder()
            .font(&font)
            .unwrap()
            .text_color(WHITE)
            .font_size(28)
            .build();

        // * @see https://docs.rs/macroquad/0.3.25/macroquad/ui/struct.Skin.html
        let ui_skin = Skin {
            window_style,
            button_style,
            label_style,
            ..root_ui().default_skin()
        };

        root_ui().push_skin(&ui_skin);
    }

    pub fn debug_touch() {
        let text = &format!("no touch - {}", now());
        let text_dimensions = measure_text(text, None, 50, 1.0);
        draw_text(
            text,
            (screen_width() / 2.0) - (text_dimensions.width / 2.0),
            100.0 + screen_height() / 2.0,
            60.0,
            YELLOW,
        );
    }

    pub fn debug_tap(init: &f64) {
        //? debug taps
        let offset = -400.0;
        let text: &str = &format!("tap registered - {init}");
        let text_dimensions = measure_text(text, None, 50, 1.0);
        draw_text(
            text,
            (screen_width() / 2.0) - (text_dimensions.width / 2.0),
            offset + 100.0 + screen_height() / 2.0,
            60.0,
            YELLOW,
        );
        let text = &format!("time - {}", now());
        let text_dimensions = measure_text(text, None, 50, 1.0);
        draw_text(
            text,
            (screen_width() / 2.0) - (text_dimensions.width / 2.0),
            offset + 160.0 + screen_height() / 2.0,
            60.0,
            YELLOW,
        );
    }

    pub fn debug_double_tap() {
        let text: &str = "double tap!";
        let text_dimensions = measure_text(text, None, 50, 1.0);
        draw_text(
            text,
            (screen_width() / 2.0) - (text_dimensions.width / 2.0),
            100.0 + screen_height() / 2.0,
            60.0,
            YELLOW,
        );
    }

    pub fn touch_window() {
        root_ui().window(
            hash!(),
            vec2(
                screen_width() / 2.0 - WINDOWS_SIZE.x / 2.0,
                screen_height() / 2.0 - WINDOWS_SIZE.y / 2.0,
            ),
            WINDOWS_SIZE,
            |ui| {
                ui.button(vec2(45.0, 75.0), "toca!");
            },
        );
    }

    pub fn main_window<F: FnOnce()>(play_func: F) {
        //todo: log on web-side
        logging::error!("jamon!");
        println!("caca!");
        debug!("caca!");

        root_ui().window(
            hash!(),
            vec2(
                screen_width() / 2.0 - WINDOWS_SIZE.x / 2.0,
                screen_height() / 2.0 - WINDOWS_SIZE.y / 2.0,
            ),
            WINDOWS_SIZE,
            |ui| {
                ui.label(vec2(80.0, -34.0), "El Juego.");
                if ui.button(vec2(45.0, 25.0), "Jugar!") {
                    play_func();
                }
                if ui.button(vec2(45.0, 125.0), "Salir!") {
                    std::process::exit(0);
                }
            },
        );
    }

    pub fn game_over_window<F: FnOnce()>(next_func: F) {
        root_ui().window(
            hash!(),
            vec2(
                screen_width() / 2.0 - WINDOWS_SIZE.x / 2.0,
                screen_height() / 2.0 - WINDOWS_SIZE.y / 2.0,
            ),
            WINDOWS_SIZE,
            |ui| {
                ui.label(vec2(80.0, -34.0), "Perdiste.");
                if ui.button(vec2(45.0, 75.0), "Menu") {
                    next_func();
                }
            },
        );
    }
}
