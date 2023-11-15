use std::ops::{Add, Sub};

use macroquad::prelude::vec2;

fn main() {
    let a = vec2(-4.0, 1.0);
    let b = vec2(3.0, 2.0);
    println!("add: {}", a + b);
    assert_eq!(a.add(b), a + b);
    println!("sub (from b to a), non-commutative: {}", a - b);
    assert_eq!(a.sub(b), vec2(-7.0, -1.0));
    assert_eq!(a.sub(b), a - b);
    assert_eq!(b.sub(a), vec2(7.0, 1.0));
    /*
     * vectors don't have origin❗
     */
    //? getting the length (or magnitude) with Pythagoras
    println!("len: {}", b.sub(a).length());
    // * distance(a,b) = ||b-a||
    assert_eq!(b.distance(a), (b - a).length());

    /*
     * unit vectors
     * aka
     * normalize vectors❗
     *
     * used for direction
     *        value
     *      ----------
     *      ||value||
     *
     * related to bunny hop issue
     */
    println!("identity vector: {}", (a / a.length()).length());
    println!("norm: {}", (a / a.length()));
    assert_eq!(a / a.length(), a.normalize());
}
