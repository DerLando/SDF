use crate::{VecType};

pub trait Abs {
    fn abs(&self) -> Self;
}

impl Abs for VecType {
    fn abs(&self) -> Self {
        match self {
            VecType::Scalar(s) => VecType::Scalar(s.abs()),
            VecType::Vec2(v) => VecType::Vec2(v.abs()),
            VecType::Vec3(v) => VecType::Vec3(v.abs()),
            VecType::Vec4(v) => VecType::Vec4(v.abs()),
        }
    }
}