use glam::Vec3;
use sdf_vecs::{VecType, ops::sub_high};

use crate::{Operator, Spatial, };

impl_binary_op!(Sub, sub);

impl Operator for Sub {
    fn operate(&self, pos: &Vec3) -> VecType {
        sub_high(&self.lhs.operate(pos), &self.rhs.operate(pos))
    }
}
