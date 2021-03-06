pub trait Delta {
    fn set_delta(&mut self, delta: f32);
}

#[derive(Default)]
pub struct ProcessDelta(pub f32);

impl Delta for ProcessDelta {
    fn set_delta(&mut self, delta: f32) {
        self.0 = delta;
    }
}

#[derive(Default)]
pub struct PhysicsDelta(pub f32);

impl Delta for PhysicsDelta {
    fn set_delta(&mut self, delta: f32) {
        self.0 = delta;
    }
}
