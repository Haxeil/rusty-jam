use bevy_ecs::component::Component;
use gdnative::{
    api::{AnimationPlayer, KinematicBody},
    core_types::Vector2,
    object::Ref,
};

#[derive(Component)]
pub struct Chicken(pub Ref<KinematicBody>);

#[derive(Component)]
pub struct Friction(pub f32);

#[derive(Component)]
pub struct Acceleration(pub f32);

#[derive(Component)]
pub struct Dog(pub Ref<KinematicBody>);

#[derive(Component)]
pub struct Velocity(pub Vector2);

#[derive(Component)]
pub struct Speed {
    pub max_speed: f32,
    pub speed: f32,
}

impl Speed {
    pub fn from(max_speed: f32) -> Speed {
        Speed {
            max_speed: max_speed,
            speed: 0.0,
        }
    }
}

#[derive(Component)]
pub struct Animation(pub Ref<AnimationPlayer>);

#[derive(Component)]
pub struct Health {
    pub max_health: f32,
    pub health: f32,
}

impl Health {
    pub fn from(max_health: f32) -> Health {
        Health {
            max_health,
            health: max_health,

        }
    }
}

