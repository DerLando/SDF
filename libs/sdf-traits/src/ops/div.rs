use glam::Vec3;
use sdf_vecs::{VecType, ops::{div_high}};

use crate::{Operator, Spatial, };

impl_binary_op!(Div, div);

impl Operator for Div {
    fn operate(&self, pos: &Vec3) -> VecType {
        div_high(&self.lhs.operate(pos), &self.rhs.operate(pos))
    }
}
