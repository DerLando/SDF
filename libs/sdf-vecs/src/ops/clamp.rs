use glam::{Vec2, Vec3, Vec4};

use crate::{VecType};

pub trait Clamp {
    fn clamp(&self, min: f32, max: f32) -> Self;
}

impl Clamp for Vec2 {
    fn clamp(&self, min: f32, max: f32) -> Self {
        Vec2::new(self.x.clamp(min, max), self.y.clamp(min, max))
    }
}

impl Clamp for Vec3 {
    fn clamp(&self, min: f32, max: f32) -> Self {
        Vec3::new(self.x.clamp(min, max), self.y.clamp(min, max), self.z.clamp(min, max))
    }
}

impl Clamp for Vec4 {
    fn clamp(&self, min: f32, max: f32) -> Self {
        Vec4::new(self.x.clamp(min, max), self.y.clamp(min, max), self.z.clamp(min, max), self.w.clamp(min, max))
    }
}

impl Clamp for VecType {
    fn clamp(&self, min: f32, max: f32) -> Self {
        match self {
            VecType::Scalar(s) => s.clamp(min, max).into(),
            VecType::Vec2(v) => v.clamp(min, max).into(),
            VecType::Vec3(v) => v.clamp(min, max).into(),
            VecType::Vec4(v) => v.clamp(min, max).into(),
        }
    }
}
