use macroquad::prelude::*;
use rapier2d::na::Vector1;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(VIOLET);
        let v = Vector1::new(2_i32);
        assert_eq!(v.x.signum(), 1);
        let v = Vector1::new(-44_i32);
        assert_eq!(v.x.signum(), -1);
        let v = Vector1::new(0_i32);
        //? on some impl, this can be 1 or error❗
        // * on math: 0 does not have direction

        let v = Vector1::new(888_i32);
        //? or length or magnitude
        assert_eq!(v.x.abs(), 888);

        // * vocabulary for SDF aka (signed distance fields)
        let a = Vector1::new(10_i32);
        let b = Vector1::new(30_i32);
        let distance = b.x.abs_diff(a.x);
        assert_eq!(distance, 20);

        next_frame().await
    }
}
