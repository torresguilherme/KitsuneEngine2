mod math;
use crate::math::vec2::Vec2;

fn main() {
    let mut result = Vec2{x:3.0, y:4.0};
    println!("{}", result.lenght());
    println!("{}", result.normalized().lenght());
    println!("{}", result.rotate(32.0).lenght());
}