// use rand::{thread_rng, Rng};
// use ggez::{
// self,
// nalgebra as na,
// };

pub fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

pub fn is_hit(invader_x: f32, invader_y: f32, invader_w: f32, invader_h: f32, fire_x: f32, fire_y:f32) -> bool {
    return fire_x < invader_x + invader_w
        && fire_x > invader_x
        && fire_y < invader_y
        && fire_y > invader_y - invader_h;
}

// pub fn randomize_vec(vec: &mut na::Vector2<f32>, x: f32, y: f32) {
//     let mut rng = thread_rng();
//     vec.x = match rng.gen_bool(0.5) {
//         true => x,
//         false => -x,
//     };
//     vec.y = match rng.gen_bool(0.5) {
//         true => y,
//         false => -y,
//     };
// }
