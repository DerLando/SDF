use glam::{Vec2, Vec3, Vec4};

/// All possible vec dimensions
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VecType {
    Scalar(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4)
}

impl From<f32> for VecType {
    fn from(arg: f32) -> Self {
        VecType::Scalar(arg)
    }
}

impl From<Vec2> for VecType {
    fn from(arg: Vec2) -> Self {
        VecType::Vec2(arg)
    }
}

impl From<Vec3> for VecType {
    fn from(arg: Vec3) -> Self {
        VecType::Vec3(arg)
    }
}

impl From<Vec4> for VecType {
    fn from(arg: Vec4) -> Self {
        VecType::Vec4(arg)
    }
}
