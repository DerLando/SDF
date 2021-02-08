use crate::{vec_1::Vec1, vec_2::Vec2, vec_3::Vec3, vec_4::Vec4};

/// All possible vec dimensions
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VecType {
    Vec1(Vec1),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4)
}

impl From<Vec1> for VecType {
    fn from(arg: Vec1) -> Self {
        VecType::Vec1(arg)
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
