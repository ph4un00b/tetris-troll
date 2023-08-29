use crate::constants::{COLUMNS, ROWS};

use macroquad::audio::{load_sound, play_sound_once};
use macroquad::{miniquad::date::now, prelude::*};

use manager::{GameMachine, Manager};
use pointers::Pointers;
use shared::{Evt, Organism, StateMachine};
use tetromino::{TetroK, Tetromino};
use ui::UI;
use universe::Universe;

mod constants;
mod manager;
mod pointers;
mod shared;
#[allow(non_snake_case)]
mod tetrio_I;
#[allow(non_snake_case)]
mod tetrio_J;
#[allow(non_snake_case)]
mod tetrio_L;
#[allow(non_snake_case)]
mod tetrio_O;
#[allow(non_snake_case)]
mod tetrio_S;
#[allow(non_snake_case)]
mod tetrio_T;
#[allow(non_snake_case)]
mod tetrio_Z;
mod tetromino;
mod ui;
mod universe;
//todo: fix shader for mobile❗
const _FRAGMENT_SHADER: &str = include_str!("background.glsl");
/*
 * Macroquad automatically adds some uniforms to shaders.
 * The ones that exist available
 *
 * _Time, Model, Projection, Texture and _ScreenTexture.
 */
const _VERTEX_SHADER: &str = "#version 100
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
    let _exit_at = 0.0;

    rand::srand(now() as u64);

    //? sound init
    //? let theme_music = load_sound("assets/bg_return_default.wav").await.unwrap();
    // let theme_music = load_sound("assets/bg_caffeine.mp3").await.unwrap();
    // let theme_music = load_sound("bg_polka.ogg").await.unwrap();
    // let theme_music = load_sound("assets/mus_picked.wav").await.unwrap();
    let transition_sound = load_sound("assets/mus_pick_item.wav").await.unwrap();
    let _dead_sound = load_sound("assets/mus_picked.wav").await.unwrap();

    UI::init().await;
    let screen_w = screen_width();
    let screen_h = screen_height();
    let screen = vec3(screen_w, screen_h, screen_w / screen_h);
    let playfield = vec2((10. * screen_h) / 32., (24. * screen_h) / 32.);
    let block: Vec2 = vec2(playfield.x / ROWS as f32, playfield.y / COLUMNS as f32);

    let tetrominos = vec![
        Tetromino::from((TetroK::I, 12., 1.)),
        Tetromino::from((TetroK::J, 12., 1.)),
        Tetromino::from((TetroK::L, 12., 1.)),
        Tetromino::from((TetroK::O, 12., 1.)),
        Tetromino::from((TetroK::S, 12., 1.)),
        Tetromino::from((TetroK::Z, 12., 1.)),
        Tetromino::from((TetroK::T, 12., 1.)),
    ];
    let mut current_tetrios = vec![Tetromino::from((TetroK::O, 12., 1.))];
    //?  Macroquad will clear the screen at the beginning of each frame.
    loop {
        clear_background(DARKPURPLE);
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

        if matches!(
            &game_state.state,
            Manager::Idle | Manager::Main | Manager::Paused | Manager::GameOver
        ) {
            Pointers::draw();
        }

        match &game_state.state {
            Manager::Idle => {
                // UI::touch_window(|| {
                //     game_state.send(&Evt::Menu);
                // });
                draw_text(
                    // format!("screen.H: {}", screen.y / MOVEMENT_SPEED).as_str(),
                    format!("screen.H: {}", screen.y).as_str(),
                    400.0,
                    20.0,
                    30.0,
                    SKYBLUE,
                );

                Universe::draw(&screen, &playfield, &block);

                if current_tetrios.is_empty() {
                    let n = rand::gen_range(0, tetrominos.len() - 1);
                    current_tetrios.push(tetrominos[n].clone())
                }

                for tetro in current_tetrios.iter_mut() {
                    tetro.update();
                    tetro.draw(&block);
                    draw_text(
                        format!("y: {}", tetro.props.y).as_str(),
                        200.0,
                        20.0,
                        30.0,
                        SKYBLUE,
                    );
                    draw_text(
                        format!("y: {}", tetro.props.y * block.y).as_str(),
                        200.0,
                        40.0,
                        30.0,
                        SKYBLUE,
                    );
                }

                current_tetrios.retain(|t| t.props.y * block.y < screen.y);
            }
            Manager::MainEntry => {
                play_sound_once(&transition_sound);
                game_state.send(&Evt::Menu);
            }
            Manager::Main => UI::main_window(&mut game_state, || Evt::Play, || Evt::Exit),
            Manager::PlayingEntry => {
                //todo: It may be a little intense that the music starts at
                //todo: full volume right away, try to lower the volume at the beginning and raise it as the game begins.
                //? play_sound(
                //?     &theme_music,
                //?     PlaySoundParams {
                //?         looped: true,
                //?         volume: 0.2,
                //?     },
                //? );
                game_state.send(&Evt::Play);
            }
            Manager::Playing => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state.send(&Evt::Pause);
                }
            }
            Manager::PlayingExit(from) => {
                //? stop_sound(&theme_music);
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

        match &game_taps {
            Evt::None => UI::debug_touch(),
            Evt::Tap(init, _delay) => UI::debug_tap(init),
            Evt::DTap => UI::debug_double_tap(),
            _ => (),
        };

        next_frame().await
    }
}
