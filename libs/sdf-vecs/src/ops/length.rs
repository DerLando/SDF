use crate::{vec_type::VecType};

pub trait Length {
    fn length(&self) -> f32;
}

impl Length for VecType {
    fn length(&self) -> f32 {
        match self {
            VecType::Scalar(s) => *s,
            VecType::Vec2(v) => v.length(),
            VecType::Vec3(v) => v.length(),
            VecType::Vec4(v) => v.length(),
        }
    }
}
