use macroquad::{
    prelude::{touches, TouchPhase, BLACK, BLUE, GREEN, WHITE, YELLOW},
    shapes::draw_circle,
};

pub struct Pointers;

impl Pointers {
    pub fn draw() {
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
    }
}
