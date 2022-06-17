use gdnative::api::*;
use gdnative::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::world::World;
use crate::ecs::sync::{resources::*, events::*};
/// The EcsSingleton "class"
#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_builder)]
pub struct EcsSingleton {
    world: World,
    schedule_process: Schedule,
    schedule_physics: Schedule,
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl EcsSingleton {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
    }

    /// The "constructor" of the class.
    fn new(_owner: &Node) -> Self {
        Self {
            world: World::new(),
            schedule_process: Schedule::default(),
            schedule_physics: Schedule::default(),
        }

    }

    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are "attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    unsafe fn _ready(&mut self, _owner: &Node) {
        self.init_resources(_owner);

        self.schedule_physics.add_stage(
            "physics",
            SystemStage::single_threaded()
  
        );

        self.schedule_process.add_stage(
            "process",
            SystemStage::single_threaded()
        );
    }

    // This function will be called in every frame
    #[export]
    unsafe fn _process(&mut self, _owner: &Node, delta: f64) {
        self.world.clear_trackers();
        update_delta_resource::<ProcessDelta>(&mut self.world, delta as f32);
        self.schedule_process.run(&mut self.world);
    }
    #[export]
    fn _physics_process(&mut self, _owner: &Node, delta: f64) {
        update_delta_resource::<PhysicsDelta>(&mut self.world, delta as f32);
        self.schedule_physics.run(&mut self.world);
    }
}


impl EcsSingleton {
    fn init_resources(&mut self, _owner: &Node) {
        self.world.init_resource::<ProcessDelta>();
        self.world.init_resource::<PhysicsDelta>();
    }
}
