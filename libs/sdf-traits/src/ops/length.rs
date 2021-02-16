use glam::Vec3;
use sdf_vecs::{VecType, ops::Length as lengthTrait};

use crate::{Operator, Spatial};

impl_unary_op!(Length, length);

impl Operator for Length {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).length().into()
    }
}
