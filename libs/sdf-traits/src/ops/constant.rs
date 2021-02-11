use sdf_vecs::{Vec1, Vec2, Vec3, Vec4, VecType};

use crate::{Operator, Spatial};


#[derive(Clone)]
pub(crate) struct Constant(pub(crate) VecType);

impl Spatial for Constant {}

impl Operator for Constant {
    fn operate(&self, _: &Vec3) -> VecType {
        self.0
    }
}

impl From<f32> for Constant {
    fn from(arg: f32) -> Self {
        Self(VecType::Vec1(Vec1::new(arg)))
    }
}

impl From<Vec1> for Constant {
    fn from(arg: Vec1) -> Self {
        Self(VecType::Vec1(arg))
    }
}

impl From<Vec2> for Constant {
    fn from(arg: Vec2) -> Self {
        Self(VecType::Vec2(arg))
    }
}

impl From<Vec3> for Constant {
    fn from(arg: Vec3) -> Self {
        Self(VecType::Vec3(arg))
    }
}

impl From<Vec4> for Constant {
    fn from(arg: Vec4) -> Self {
        Self(VecType::Vec4(arg))
    }
}