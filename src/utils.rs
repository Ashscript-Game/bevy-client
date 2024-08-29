use std::f32::consts::PI;

use bevy::math::Vec3;
use hexx::Hex;
use rand::Rng;

pub fn find_angle(v1: &Vec3, v2: &Vec3) -> f32 {

    let x_diff = v2.x - v1.x;
    let y_diff = v2.y - v1.y;

    y_diff.atan2(x_diff) + PI / 2.
}

pub fn find_angle_coords(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {

    let x_diff = x2 - x1;
    let y_diff = y2 - y1;

    y_diff.atan2(x_diff) + PI / 2.
}

pub fn signed_distance(pos1: Vec3, pos2: Vec3) -> f32 {
    let dx = (pos1.x - pos2.x).abs();
    let dy = (pos1.y - pos2.y).abs();

    if (dx >= 0.) ^ (dy >= 0.) {
        return -(-dx).min(-dy);
    }

    -dx.max(dy)

    // let sign = if pos1.x > pos2.x && pos1.y > pos2.y {
    //     -1.
    // } else if pos1.x < pos2.x && pos1.y < pos2.y {
    //     1.

    // } else {
    //     (-dx - dy).signum()
    //     /* 0. */
    // };
    // sign * (dx + dy)
}

pub fn pick<T>(array: &[T]) -> &T {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..array.len());

    &array[index]
}

pub mod hex {
    use hexx::Hex;

    pub fn pack(hex: Hex) -> i32 {
        hex.x * i32::MAX + hex.y
    }

    pub fn unpack(packed: i32) -> Hex {
        let x = packed % i32::MAX;
        let y = packed - x;

        Hex::new(x, y)
    }
}