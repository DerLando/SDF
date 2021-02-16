use glam::{Vec2, Vec3, Vec4};

use crate::{component_access::ComponentAccess, dimension::Dimension, vec_type::VecType};

/// Scale between the different dimensions of vectors
/// Under the hood this uses the wrapping of `ComponentAccess`.
pub trait Scale {
    fn scale1(&self) -> f32;
    fn scale2(&self) -> Vec2;
    fn scale3(&self) -> Vec3;
    fn scale4(&self) -> Vec4;
}

impl Scale for VecType {
    fn scale1(&self) -> f32 {
        match self.dimension() {
            1 | 2 | 3 | 4 => self.x(),
            _ => unreachable!()
        }
    }

    fn scale2(&self) -> Vec2 {
        match self.dimension() {
            1 | 2 | 3 | 4 => Vec2::new(self.x(), self.y()),
            _ => unreachable!()
        }
    }

    fn scale3(&self) -> Vec3 {
        match self.dimension() {
            1 | 2 | 3 | 4 => Vec3::new(self.x(), self.y(), self.z()),
            _ => unreachable!()
        }
    }

    fn scale4(&self) -> Vec4 {
        match self.dimension() {
            1 | 2 | 3 | 4 => Vec4::new(self.x(), self.y(), self.z(), self.w()),
            _ => unreachable!()
        }
    }
}

pub(crate) fn scale_n<S: Scale>(vec: &S, dimension: u8) -> VecType {
    match dimension {
        1 => vec.scale1().into(),
        2 => vec.scale2().into(),
        3 => vec.scale3().into(),
        4 => vec.scale4().into(),
        _ => unreachable!()
    }
}