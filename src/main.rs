use crate::constants::{PLAYFIELD_H, PLAYFIELD_W};

use bloque::Bloque;

use macroquad::audio::{load_sound, play_sound_once};
use macroquad::{miniquad::date::now, prelude::*};

use manager::{GameMachine, Manager};
use physics::{Physics, PhysicsEvent};
use piso::Piso;
use pointers::Pointers;

use shared::{Evt, Organism, Position, StateMachine};
use tetromino::{TetroK, Tetromino};
use ui::UI;
use world::World;

mod bloque;
mod constants;
mod debug;
mod game_configs;
mod manager;
mod physics;
mod piso;
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
mod world;
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
    let block: Vec2 = vec2(
        playfield.x / PLAYFIELD_W as f32,
        playfield.y / PLAYFIELD_H as f32,
    );

    //?  Macroquad will clear the screen at the beginning of each frame.
    let mut world = World::new(Physics::new(), block, screen, playfield);
    let tetrominos = vec![
        Tetromino::from(TetroK::I),
        Tetromino::from(TetroK::J),
        Tetromino::from(TetroK::L),
        Tetromino::from(TetroK::O),
        Tetromino::from(TetroK::S),
        Tetromino::from(TetroK::Z),
        Tetromino::from(TetroK::T),
    ];
    let mut current_tetrios = vec![Tetromino::from(TetroK::O)];
    let mut physics_events: Vec<PhysicsEvent> = Vec::new();

    let restitution = 0.8;
    let mut bloque = Bloque::new(
        &mut world,
        vec2(12. * block.x, 2. * block.y),
        10.,
        restitution,
    );
    let mut bloque2 = Bloque::new(
        &mut world,
        vec2(13.1 * block.x, 2. * block.y),
        10.5,
        restitution * 1.8,
    );
    let mut bloque3 = Bloque::new(
        &mut world,
        vec2(14.1 * block.x, 2. * block.y),
        10.,
        restitution * 2.2,
    );
    let mut piso = Piso::new(
        &mut world,
        vec2(0.5 * (screen.x - (20. * block.x)), 31. * block.y),
        vec2(20. * block.x, 1. * block.x),
    );

    // let mut c = 0;
    let mut tetro_x = 0_usize;
    let mut tetro_x2 = 0.0_f32;
    let mut tetro_y = 0.0_f32;
    let mut g_piece = 0;
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
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                // todo: recheck las ui's❗
                // UI::touch_window(|| {
                //     game_state.send(&Evt::Menu);
                // });
                // Universe::draw(&screen, &playfield, &block);
                world.render();

                if current_tetrios.is_empty() {
                    // let n = rand::gen_range(0, tetrominos.len() - 1);
                    g_piece += 1;
                    current_tetrios.push(tetrominos[g_piece % 7].clone());
                }

                for tetro in current_tetrios.iter_mut() {
                    tetro.update(&mut world, &mut physics_events);
                    tetro.draw(&mut world);
                    tetro_x = tetro.playfield_x;
                    tetro_x2 = tetro.props.x;
                    tetro_y = tetro.props.y;
                    if tetro.props.y * block.y >= (screen.y * 1.0) {
                        world.add(tetro);
                    }
                }

                current_tetrios.retain(|t| t.props.y * block.y < (screen.y * 1.0));

                bloque.update(&mut world, &mut physics_events);
                bloque.draw(&mut world);

                bloque2.update(&mut world, &mut physics_events);
                bloque2.draw(&mut world);

                bloque3.update(&mut world, &mut physics_events);
                bloque3.draw(&mut world);
                piso.draw(&mut world);

                world.physics.update(get_frame_time(), &mut physics_events);
                world.physics.draw_colliders();

                egui_macroquad::ui(|egui_ctx| {
                    egui::Window::new("❤ debug").show(egui_ctx, |ui| {
                        ui.label(format!("screen.H: {}", world.screen.y));
                        ui.label(format!("screen.W: {}", world.screen.x));
                        ui.label(format!("y: {}", tetro_y));
                        ui.label(format!("y: {}", tetro_y * world.block.y));
                        ui.label(format!("altura: {}", bloque.y()));
                        //? x handler
                        ui.label(format!("x: {tetro_x}",));
                        ui.label(format!("x playfield: {tetro_x2}"));
                    });
                });
                egui_macroquad::draw();
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
