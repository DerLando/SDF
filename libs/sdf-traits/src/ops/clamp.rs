use sdf_vecs::ops::Clamp as ClampTrait;

use crate::{Operator, Spatial};

pub(crate) struct Clamp {
    value: Box<dyn Spatial>,
    min: f32,
    max: f32
}

impl Operator for Clamp {
    fn operate(&self, pos: &sdf_vecs::Vec3) -> sdf_vecs::VecType {
        self.value.operate(pos).clamp(self.min, self.max)
    }
}

pub(crate)fn clamp(sdf: impl Spatial + 'static, min: f32, max: f32) -> Clamp {
    Clamp {
        value: Box::new(sdf),
        min,
        max
    }
}