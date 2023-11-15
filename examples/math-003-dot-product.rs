use nalgebra::{ComplexField, Vector2};

fn main() {
    let a = Vector2::new(4, 8);
    let res = a * 5;
    println!("multiplication: {res}");

    //? Dot product
    //? aka
    //? Scalar product (bc you are getting an scalar aka a fancy name for numbers❗)
    //? aka
    //? Symmetric product
    //? aka
    //? Interior product
    /*
     *
     * A dot B = A.x * B.x + A.y * B.y
     *
     */
    //? Getting a scalar projection (Û -> normalized vector, they live within the unit circle)
    /*
     *
     * USE CASES:
     * 1. relationship of vectors (for instance: loudness of an object, normal & velocity vector):
     *  (-) behind,
     *  (+) in front
     * of something
     *
     * 2. get how far an object is?
     * Û dot V = U.x * V.x + U.y * V.y
     *
     */
    let u = Vector2::new(6.0, 7.0);
    let v = Vector2::new(4.0, 5.0);
    let u_norm = u.normalize();
    println!("u: {u}, u_norm: {u_norm}");
    let res = u.dot(&v);
    println!("dot: {res}");

    let v = Vector2::new(4.0, -5.0);
    let res = u_norm.dot(&v);
    println!("dot: {res}");

    //? getting the angle
    let v_norm = v.normalize();
    let res = u_norm.dot(&v_norm).acos();
    println!("angle = {res}");

    let v = Vector2::new(4.0, 5.0);
    let v_norm = v.normalize();
    let res = u_norm.dot(&v_norm).acos();
    println!("angle2 = {res}");

    let a = Vector2::new(0.0, 10.0).normalize();
    let b = Vector2::new(0.0, -10.0).normalize();
    let res = a.dot(&b).acos();
    println!("angle3 = {res}");
}
