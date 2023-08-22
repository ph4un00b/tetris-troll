use macroquad::prelude::Rect;

use crate::shared::{Collision, Coso};

pub struct Enemy {
    pub props: Coso,
}

impl Enemy {
    pub fn new(props: Coso) -> Self {
        Self { props }
    }
}

impl Collision for Enemy {
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
