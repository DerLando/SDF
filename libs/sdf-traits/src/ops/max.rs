use glam::Vec3;
use sdf_vecs::{VecType, ops::max_high};

use crate::{Operator, Spatial};

impl_binary_op!(Max, max);

impl Operator for Max {
    fn operate(&self, pos: &Vec3) -> VecType {
        max_high(&self.lhs.operate(pos), &self.rhs.operate(pos))
    }
}
