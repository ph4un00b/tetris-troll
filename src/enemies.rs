use macroquad::{
    prelude::{rand, vec2, Vec2, GREEN, ORANGE, PINK, VIOLET},
    shapes::draw_rectangle,
    time::get_frame_time,
    window::screen_width,
};
use macroquad_particles::{ColorCurve, Emitter, EmitterConfig};

use crate::{
    enemy::Enemy,
    player::Player,
    shared::{Collision, Coso, Organism},
};

pub struct Enemies {
    pub soldiers: Vec<Enemy>,
    animations: Vec<(Emitter, Vec2)>,
}

impl Organism for Enemies {
    fn reset(&mut self) {
        self.soldiers.clear();
        self.animations.clear();
    }

    fn update(&mut self) {
        let delta_time = get_frame_time();
        //? instances
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            self.soldiers.push(Enemy::new(Coso {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
                collided: false,
            }));
        }
        //? move instances
        for enemy in &mut self.soldiers {
            enemy.props.y += enemy.props.speed * delta_time;
        }
        //? optimization: Remove self.soldiers below bottom of screen
        self.soldiers.retain(|square| !square.props.collided);
        self.soldiers
            .retain(|square| square.props.y < screen_width() + square.props.size);
        // todo hay un problema con eliminar muy rápido la explosiones
        // explosions.retain(|(explosion, _)| explosion.config.emitting);
    }

    fn draw(&mut self) {
        for (explosion, coords) in self.animations.iter_mut() {
            explosion.draw(*coords);
        }
        for cosito in &self.soldiers {
            draw_rectangle(
                cosito.props.x - cosito.props.size / 2.0,
                cosito.props.y - cosito.props.size / 2.0,
                cosito.props.size,
                cosito.props.size,
                PINK,
            );
        }
    }
}

impl Enemies {
    pub fn collides_with(&mut self, player: &mut Player) -> bool {
        let mut result = false;
        for square in self.soldiers.iter_mut() {
            if !player.props.collided && player.collides_with(&square.rect()) {
                result = true;
                player.props.collided = true;
                square.props.collided = true;
                self.animations.push((
                    Emitter::new(EmitterConfig {
                        amount: square.props.size.round() as u32 * 4,
                        ..particle_explosion()
                    }),
                    vec2(square.props.x, square.props.y),
                ));
            }
        }
        result
    }

    pub fn new() -> Self {
        Self {
            soldiers: vec![],
            animations: vec![],
        }
    }
}

fn particle_explosion() -> EmitterConfig {
    /*
     * We will use the same configuration for all explosions,
     * and only will resize it based on the size of the square.props.
     *
     * Therefore, we create a function returning one EmitterConfig which
     * can be used to create one Emitter. One Emitter is a point based on
     * particles can be generated.
     *
     * @see https://docs.rs/macroquad-particles/latest/macroquad_particles/struct.EmitterConfig.html
     */
    EmitterConfig {
        local_coords: false,
        one_shot: true,
        emitting: true,
        lifetime: 0.6,
        lifetime_randomness: 0.3,
        explosiveness: 0.65,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 300.0,
        initial_velocity_randomness: 0.8,
        size: 3.0,
        size_randomness: 0.3,
        colors_curve: ColorCurve {
            start: VIOLET,
            mid: ORANGE,
            end: GREEN,
        },
        ..Default::default()
    }
}
