// // * @see https://github.com/0x61nas/todo2#using-the-log-feature
// #[macro_use]
// extern crate todo2;
// // #[macro_use]
// // extern crate log;
// use simple_logger::SimpleLogger;

// * game deps

use crate::constants::{DEBUG_GROUND, PLAYFIELD_H, PLAYFIELD_W};

use bloque::Bloque;

use constants::NUMBER_OF_TETROMINOS;
use egui::Pos2;
use macroquad::audio::{load_sound, play_sound_once};
use macroquad::{miniquad::date::now, prelude::*};

use manager::{GameMachine, Manager};
use physics::{Physics, PhysicsEvent};
use piso::Piso;
use pointers::Pointers;

use shared::{Evt, Organism, PanelLayout, Position, StateMachine, WindowPanel};
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
mod world_with_holes;
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
    // * @see https://github.com/0x61nas/todo2#using-the-log-feature
    // SimpleLogger::new().init().unwrap();

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
        Tetromino::from(TetroK::I, &world),
        Tetromino::from(TetroK::J, &world),
        Tetromino::from(TetroK::L, &world),
        Tetromino::from(TetroK::O, &world),
        Tetromino::from(TetroK::S, &world),
        Tetromino::from(TetroK::Z, &world),
        Tetromino::from(TetroK::T, &world),
    ];
    let mut current_tetro = vec![Tetromino::from(TetroK::L, &world)];
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
    let mut ground = Piso::new(
        &mut world,
        vec2(0.5 * (screen.x - (20. * block.x)), 31. * block.y),
        vec2(20. * block.x, 1. * block.x),
    );

    let mut g_piece = 0_usize;
    let g_floor_y = (world.screen.y * 0.2) + world.playfield.y;

    let mut debug_layout = PanelLayout::new(vec2(10.0, screen_height() * 0.5), 100.0);
    let mut debug_window = WindowPanel::new(
        "Debug!".to_string(),
        vec2(screen_width() * 0.75, screen_height() * 0.5),
        100.0,
    );

    loop {
        if cfg!(unix) || cfg!(windows) {
            clear_background(VIOLET);
        } else {
            clear_background(DARKBLUE);
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

        if matches!(
            &game_state.state,
            Manager::Idle | Manager::Main | Manager::Paused | Manager::GameOver
        ) {
            Pointers::draw();
        }

        if (cfg!(unix) || cfg!(windows)) && is_key_released(KeyCode::R) {
            //? poor's man hot reload 😏
            std::process::Command::new("cargo")
                .arg("run")
                .current_dir("Z:/projects/tetris-troll")
                .spawn()
                .expect("fallo el hot reload!");
            panic!("algo no paso!")
        };

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

                if current_tetro.is_empty() {
                    if cfg!(unix) || cfg!(windows) {
                        g_piece += 1;
                        current_tetro.push(tetrominos[g_piece % NUMBER_OF_TETROMINOS].clone());
                    } else {
                        let n = rand::gen_range(0, NUMBER_OF_TETROMINOS);
                        current_tetro.push(tetrominos[n].clone());
                    };
                }

                for tetro in current_tetro.iter_mut() {
                    // println!("floor-y: {}", g_floor_y - tetro.props.size.y);
                    world.render(g_floor_y - tetro.props.size.y);
                    tetro.update(&mut world, &mut physics_events);
                    tetro.draw(&mut world);

                    {
                        debug_window.draw(|| {
                            vec![
                                format!("min x: {}", tetro.props.min_x),
                                format!("max x: {}", tetro.props.max_x),
                            ]
                        });
                    }
                    {
                        debug_layout.row(0);
                        debug_layout.text(format!(
                            "coord: {}, {}",
                            tetro.playfield.coord.x, tetro.playfield.coord.y
                        ));
                    }
                    {
                        debug_layout.row(1);
                        debug_layout.text(format!(
                            "size: {}, {}",
                            tetro.playfield.size.x, tetro.playfield.size.y
                        ));
                    }
                    {
                        debug_layout.row(2);
                        debug_layout.text(format!("props: {}, {}", tetro.props.x, tetro.props.y));
                    }

                    if tetro
                        .process_current_positions(|x, y, _value| {
                            if cfg!(unix) || cfg!(windows) {
                                (world.floor[x][y] == DEBUG_GROUND).then_some(())
                            } else {
                                // * wasm: mobile touch this adds up instantly
                                //todo!("add delay", by: 2023-10-01);
                                (world.floor[x][y + 1] == DEBUG_GROUND).then_some(())
                            }
                        })
                        .is_break()
                    {
                        //? remove painted pieces
                        for (x, row) in world.floor.clone().iter().enumerate() {
                            for (y, _value) in row.iter().enumerate() {
                                if world.floor[x][y] == 6_u8 {
                                    world.floor[x][y] = 0_u8;
                                }
                            }
                        }
                        world.merge(tetro);
                    }
                }

                {
                    // * from @link https://discord.com/channels/710177966440579103/710180051349405746/1067069758329073664
                    let (mx, my) = mouse_position();
                    debug_layout.row(3);
                    debug_layout.text(format!("mouse: {}, {}", mx, my));
                }

                current_tetro.retain(|tetro| tetro.in_game);

                bloque.update(&mut world, &mut physics_events);
                bloque.draw(&mut world);

                bloque2.update(&mut world, &mut physics_events);
                bloque2.draw(&mut world);

                bloque3.update(&mut world, &mut physics_events);
                bloque3.draw(&mut world);
                ground.draw(&mut world);

                world.physics.update(get_frame_time(), &mut physics_events);
                world.physics.draw_colliders();
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
