use godot::engine::AnimatedSprite2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=AnimatedSprite2D)]
struct Dude {
    speed: f64,
    angular_speed: f64,

    #[base]
    sprite: Base<AnimatedSprite2D>,
}
