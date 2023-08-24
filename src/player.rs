use macroquad::{
    prelude::Rect,
    window::{screen_height, screen_width},
};

use crate::shared::{Collision, Coso, Organism};

pub struct Player {
    pub props: Coso,
}

impl Player {
    pub fn new(props: Coso) -> Self {
        Self { props }
    }
}

impl Organism for Player {
    fn update(&mut self) {}

    fn draw(&mut self) {}

    fn reset(&mut self) {
        self.props.collided = false;
        self.props.x = screen_width() / 2.0;
        self.props.y = screen_height() / 2.0;
    }
}

impl Collision for Player {
    fn collides_with(&self, other: &Rect) -> bool {
        self.rect().overlaps(other)
    }

    //? el cuadro que mapea la colisión❗
    /*
     * Rect also starts from the upper left corner, so we must too here subtract half
     * the stork from both X and Y.
     *
     * phau: falta un debug mode para ver el perímetro❗
     */
    //todo: draw helpers
    fn rect(&self) -> Rect {
        Rect {
            x: self.props.x - self.props.size / 2.0,
            y: self.props.y - self.props.size / 2.0,
            w: self.props.size,
            h: self.props.size,
        }
    }
}
