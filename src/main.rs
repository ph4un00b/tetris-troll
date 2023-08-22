use constants::MOVEMENT_SPEED;
use enemies::Enemies;

use macroquad::audio::{load_sound, play_sound, play_sound_once, stop_sound, PlaySoundParams};

use macroquad::{miniquad::date::now, prelude::*};

use shared::Organism;
use ui::UI;

use crate::player::Player;
use crate::shared::Coso;
mod constants;
mod enemies;
mod enemy;
mod player;
mod shared;
mod ui;
//todo: fix shader for mobile❗
const FRAGMENT_SHADER: &str = include_str!("starfield-shader.glsl");

/*
 * Macroquad automatically adds some uniforms to shaders.
 * The ones that exist available
 *
 * _Time, Model, Projection, Texture and _ScreenTexture.
 */
const VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
}
";

enum GS {
    Main,
    Playing,
    Paused,
    GameOver,
}

#[derive(PartialEq)]
enum Evt {
    None,
    Tapped(f64, f64),
    DoubleTapped,
}
#[macroquad::main("TetrisTroll")]
async fn main() {
    simulate_mouse_with_touch(true);

    //? game shader init
    let direction_modifier: f32 = 0.0;
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    let material = load_material(
        ShaderSource {
            glsl_vertex: Some(VERTEX_SHADER),
            glsl_fragment: Some(FRAGMENT_SHADER),
            metal_shader: None,
        },
        MaterialParams {
            uniforms: vec![
                ("iResolution".to_owned(), UniformType::Float2),
                ("direction_modifier".to_owned(), UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();
    //? game inits
    let mut game_state = GS::Main;
    let mut game_taps = Evt::None;
    let mut game_over_at = 0.0;

    rand::srand(now() as u64);
    let mut enemies = Enemies::new();
    let mut player = Player::new(Coso {
        size: 52.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
    });

    let x = screen_width() / 2.0;
    let y = screen_height() / 2.0;

    //? sound init
    let theme_music = load_sound("assets/bg_return_default.wav").await.unwrap();
    // let theme_music = load_sound("assets/bg_caffeine.mp3").await.unwrap();
    // let theme_music = load_sound("bg_polka.ogg").await.unwrap();
    // let theme_music = load_sound("assets/mus_picked.wav").await.unwrap();
    let transition_sound = load_sound("assets/mus_pick_item.wav").await.unwrap();
    let dead_sound = load_sound("assets/mus_picked.wav").await.unwrap();
    let mut main_music_started = false;
    UI::init().await;
    //?  Macroquad will clear the screen at the beginning of each frame.
    loop {
        clear_background(DARKPURPLE);
        //?shader
        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", direction_modifier);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();
        //? end-shader

        for touch in touches() {
            if let TouchPhase::Started = touch.phase {
                game_taps = match game_taps {
                    Evt::None => Evt::Tapped(now(), 0.250),
                    Evt::Tapped(init, delay) if now() > (init + delay) => Evt::Tapped(now(), delay),
                    Evt::Tapped(_, _) => Evt::DoubleTapped,
                    Evt::DoubleTapped => Evt::Tapped(now(), 0.250),
                };
            };
        }

        match &game_taps {
            Evt::None => UI::debug_touch(),
            Evt::Tapped(init, _delay) => UI::debug_tap(init),
            Evt::DoubleTapped => UI::debug_double_tap(),
        };

        if let (GS::Main | GS::Paused | GS::GameOver, Evt::None | Evt::Tapped(_, _)) =
            (&game_state, &game_taps)
        {
            for touch in touches() {
                let (fill_color, size) = match touch.phase {
                    TouchPhase::Started => (GREEN, 20.0),
                    TouchPhase::Stationary => (WHITE, 20.0),
                    TouchPhase::Moved => (YELLOW, 20.0),
                    TouchPhase::Ended => (BLUE, 20.0),
                    TouchPhase::Cancelled => (BLACK, 20.0),
                };
                draw_circle(touch.position.x, touch.position.y, size, fill_color);
            }
        };
        match (&game_state, &game_taps) {
            (GS::Main, Evt::None) => {
                UI::touch_window();
            }
            (GS::Main, Evt::Tapped(_, _)) => UI::main_window(|| {
                enemies.reset();
                player.reset();
                game_state = GS::Playing;
                game_taps = Evt::None;
            }),
            (GS::Main, Evt::DoubleTapped) => {}
            (GS::Playing, Evt::None | Evt::Tapped(_, _)) => {
                if !main_music_started {
                    main_music_started = true;
                    //todo: It may be a little intense that the music starts at
                    //todo: full volume right away, try to lower the volume at the beginning and raise it as the game begins.
                    play_sound(
                        &theme_music,
                        PlaySoundParams {
                            looped: true,
                            volume: 0.2,
                        },
                    );
                }

                enemies.update();
                player.update();

                //? PAUSE on ESC❗
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GS::Paused;
                }

                //? world
                draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
                draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
                draw_circle(x - 30.0, y - 30.0, 45.0, BROWN);
                draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
                if enemies.collides_with(&mut player) {
                    play_sound_once(&dead_sound);
                    game_over_at = now() + 1.25;
                }
                if player.props.collided && now() > game_over_at {
                    game_state = GS::GameOver;
                }
                enemies.draw();
                player.draw();
            }
            (GS::Playing, Evt::DoubleTapped) => {
                play_sound_once(&transition_sound);
                stop_sound(&theme_music);
                game_state = GS::Paused;
                game_taps = Evt::None;
            }
            (GS::Paused, Evt::None | Evt::Tapped(_, _)) => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GS::Playing;
                }
                let text = "Pausad";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            }
            (GS::Paused, Evt::DoubleTapped) => {
                play_sound_once(&transition_sound);
                main_music_started = false;
                game_state = GS::Playing;
                game_taps = Evt::None;
            }
            (GS::GameOver, Evt::None | Evt::Tapped(_, _)) => {
                stop_sound(&theme_music);
                main_music_started = false;
                UI::game_over_window(|| {
                    game_state = GS::Main;
                    game_taps = Evt::Tapped(now(), 0.250);
                })
            }
            (GS::GameOver, Evt::DoubleTapped) => {}
        }

        next_frame().await
    }
}
