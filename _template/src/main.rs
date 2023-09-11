use crate::player::Player;
use crate::shared::Coso;
use constants::MOVEMENT_SPEED;
use macroquad::audio::{load_sound, play_sound, play_sound_once, stop_sound, PlaySoundParams};
use macroquad::{miniquad::date::now, prelude::*};

use manager::{GameMachine, Manager};
use pointers::Pointers;
use shared::{Evt, StateMachine};
use ui::UI;

mod constants;
mod manager;
mod player;
mod pointers;
mod shared;
mod ui;
mod universe;
//todo: fix shader for mobile❗
const FRAGMENT_SHADER: &str = include_str!("background.glsl");
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

#[macroquad::main("TetrisTroll")]
async fn main() {
    simulate_mouse_with_touch(true);

    //? game shader init
    // let direction_modifier: f32 = 0.0;
    // let render_target = render_target(320, 150);
    // render_target.texture.set_filter(FilterMode::Nearest);
    // let material = load_material(
    //     ShaderSource {
    //         glsl_vertex: Some(VERTEX_SHADER),
    //         glsl_fragment: Some(FRAGMENT_SHADER),
    //         metal_shader: None,
    //     },
    //     MaterialParams {
    //         uniforms: vec![
    //             ("iResolution".to_owned(), UniformType::Float2),
    //             ("direction_modifier".to_owned(), UniformType::Float1),
    //         ],
    //         ..Default::default()
    //     },
    // )
    // .unwrap();
    //? game inits
    let mut game_state = GameMachine::new().await;
    let mut game_taps = Evt::None;
    let mut exit_at = 0.0;

    rand::srand(now() as u64);
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

    UI::init().await;
    //?  Macroquad will clear the screen at the beginning of each frame.
    loop {
        if cfg!(unix) || cfg!(windows) {
            clear_background(DARKBLUE);
        } else {
            clear_background(DARKPURPLE);
        };
        //?shader
        // material.set_uniform("iResolution", (screen_width(), screen_height()));
        // material.set_uniform("direction_modifier", direction_modifier);
        // gl_use_material(&material);
        // draw_texture_ex(
        //     &render_target.texture,
        //     0.,
        //     0.,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(vec2(screen_width(), screen_height())),
        //         ..Default::default()
        //     },
        // );
        // gl_use_default_material();
        //? end-shader

        for touch in touches() {
            if let TouchPhase::Started = touch.phase {
                game_taps = match game_taps {
                    Evt::None => Evt::Tap(now(), 0.250),
                    Evt::Tap(init, delay) if now() > (init + delay) => Evt::Tap(now(), delay),
                    Evt::Tap(_, _) => {
                        game_state.send(&Evt::DTap);
                        Evt::DTap
                    }
                    Evt::DTap => Evt::Tap(now(), 0.250),
                    _ => Evt::None,
                }
            };
        }

        match &game_taps {
            Evt::None => UI::debug_touch(),
            Evt::Tap(init, _delay) => UI::debug_tap(init),
            Evt::DTap => UI::debug_double_tap(),
            _ => (),
        };

        if matches!(
            &game_state.state,
            Manager::Idle | Manager::Main | Manager::Paused | Manager::GameOver
        ) {
            Pointers::draw();
        }

        match &game_state.state {
            Manager::Idle => UI::touch_window(|| {
                game_state.send(&Evt::Menu);
            }),
            Manager::MainEntry => {
                play_sound_once(&transition_sound);
                game_state.send(&Evt::Menu);
            }
            Manager::Main => UI::main_window(&mut game_state, || Evt::Play, || Evt::Exit),
            Manager::PlayingEntry => {
                //todo: It may be a little intense that the music starts at
                //todo: full volume right away, try to lower the volume at the beginning and raise it as the game begins.
                play_sound(
                    &theme_music,
                    PlaySoundParams {
                        looped: true,
                        volume: 0.2,
                    },
                );
                game_state.send(&Evt::Play);
            }
            Manager::Playing => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state.send(&Evt::Pause);
                }
            }
            Manager::PlayingExit(from) => {
                stop_sound(&theme_music);
                if matches!(from, Evt::Dead) {
                    game_state.send(&Evt::Dead);
                } else {
                    game_state.send(&Evt::Pause);
                };
            }
            Manager::PausedEntry => {
                play_sound_once(&transition_sound);
                game_state.send(&Evt::Pause);
            }
            Manager::Paused => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state.send(&Evt::Play);
                }
                UI::game_paused();
            }
            Manager::GameOver => UI::game_over_window(|| {
                game_state.send(&Evt::Menu);
            }),
            Manager::Exit => std::process::exit(0),
        };

        next_frame().await
    }
}
