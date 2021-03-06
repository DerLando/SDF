use std::fmt::Display;

use glam::{Vec2, Vec3, Vec4};

/// All possible vec dimensions
#[derive(Debug, PartialEq, Clone, Copy,)]
pub enum VecType {
    Scalar(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4)
}

impl Display for VecType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VecType::Scalar(s) => write!(f, "{}", *s),
            VecType::Vec2(v) => write!(f, "({}, {})", v.x, v.y),
            VecType::Vec3(v) => write!(f, "({}, {}, {})", v.x, v.y, v.z),
            VecType::Vec4(v) => write!(f, "({}, {}, {}, {})", v.x, v.y, v.z, v.w),
        }
    }
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
