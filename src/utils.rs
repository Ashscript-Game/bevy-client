use bevy::math::Vec3;
use rand::Rng;

pub fn find_angle(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    /*         let x_diff = (x1 - x2).abs();
    let y_diff = (y1 - y2).abs(); */
    let x_diff = /* (x2 - x1).abs() */(x2 - x1);
    let y_diff = /* (y2 - y1).abs() */(y2 - y1);

    y_diff.atan2(x_diff)
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