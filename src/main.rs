use crate::player::Player;
use crate::shared::Coso;
use constants::MOVEMENT_SPEED;
use enemies::Enemies;
use macroquad::audio::{load_sound, play_sound, play_sound_once, stop_sound, PlaySoundParams};
use macroquad::{miniquad::date::now, prelude::*};
use pointers::Pointers;
use shared::Organism;
use ui::UI;
use universe::Universe;
mod constants;
mod enemies;
mod enemy;
mod player;
mod pointers;
mod shared;
mod ui;
mod universe;
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
    Dead,
}

#[derive(PartialEq)]
enum Evt {
    None,
    Tap(f64, f64),
    DTap,
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
                    Evt::None => Evt::Tap(now(), 0.250),
                    Evt::Tap(init, delay) if now() > (init + delay) => Evt::Tap(now(), delay),
                    Evt::Tap(_, _) => Evt::DTap,
                    Evt::DTap => Evt::Tap(now(), 0.250),
                };
            };
        }

        match &game_taps {
            Evt::None => UI::debug_touch(),
            Evt::Tap(init, _delay) => UI::debug_tap(init),
            Evt::DTap => UI::debug_double_tap(),
        };

        if let (GS::Main | GS::Paused | GS::Dead, Evt::None | Evt::Tap(_, _)) =
            (&game_state, &game_taps)
        {
            Pointers::draw();
        };

        match (&game_state, &game_taps) {
            (GS::Main, Evt::None) => {
                UI::touch_window();
            }
            (GS::Main, Evt::Tap(_, _)) => UI::main_window(|| {
                enemies.reset();
                player.reset();
                game_state = GS::Playing;
                game_taps = Evt::None;
            }),
            (GS::Main, Evt::DTap) => {}
            (GS::Playing, Evt::None | Evt::Tap(_, _)) => {
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
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GS::Paused;
                }
                if enemies.collides_with(&mut player) {
                    play_sound_once(&dead_sound);
                    game_over_at = now() + 1.25;
                }
                if player.props.collided && now() > game_over_at {
                    game_state = GS::Dead;
                }
                enemies.draw();
                player.draw();
                Universe::draw();
            }
            (GS::Playing, Evt::DTap) => {
                play_sound_once(&transition_sound);
                stop_sound(&theme_music);
                game_state = GS::Paused;
                game_taps = Evt::None;
            }
            (GS::Paused, Evt::None | Evt::Tap(_, _)) => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GS::Playing;
                }
                UI::game_paused();
            }
            (GS::Paused, Evt::DTap) => {
                play_sound_once(&transition_sound);
                main_music_started = false;
                game_state = GS::Playing;
                game_taps = Evt::None;
            }
            (GS::Dead, Evt::None | Evt::Tap(_, _)) => {
                stop_sound(&theme_music);
                main_music_started = false;
                UI::game_over_window(|| {
                    game_state = GS::Main;
                    game_taps = Evt::Tap(now(), 0.250);
                })
            }
            (GS::Dead, Evt::DTap) => {}
        }

        next_frame().await
    }
}
