

use gdnative::api::*;
use gdnative::prelude::*;


/// The EcsSingleton "class"
#[derive(NativeClass, ToVariant, FromVariant)]
#[inherit(KinematicBody)]
#[register_with(Self::register_builder)]
pub struct Chicken {

    velocity: Vector3,

    speed: f32,

    #[property]
    max_speed: f32,

    health: f32,

    #[property]
    max_health: f32,

    #[property]
    animation_player: Option<Ref<AnimationPlayer>>,


}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl Chicken {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {}

    /// The "constructor" of the class.
    fn new(_owner: &KinematicBody) -> Self {
        Self { 
            velocity: Vector3::ZERO, 
            speed: 0.0, 
            max_speed: 0.0, 
            health: 0.0, 
            max_health: 100.0,
            animation_player: None,
        }

    }

    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are "attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    unsafe fn _ready(&mut self, owner: &KinematicBody) {
        godot_print!("Ayour YA Zamel");
        let ecs = owner.get_node_as::<Node>("/root/EcsSingleton");

        if let Some(ecs) = ecs {
            ecs.call_deferred("add_chicken_to_ecs", &[

                    owner.assume_shared().to_variant(),

                    self.animation_player.to_variant(),

                    self.max_speed.to_variant(), 

                    self.max_health.to_variant(), 

                ]);

        }


    }

    // This function will be called in every frame
  }
