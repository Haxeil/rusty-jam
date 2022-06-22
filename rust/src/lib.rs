mod ecs;
mod ecs_singleton;
mod classes;

use classes::*;
use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<ecs_singleton::EcsSingleton>();
    handle.add_class::<chicken::Chicken>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
