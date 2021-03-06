use glam::Vec3;
use sdf_vecs::{VecType, ops::{mul_high}};

use crate::{Operator, Spatial, };

impl_binary_op!(Mul, mul);

impl Operator for Mul {
    fn operate(&self, pos: &Vec3) -> VecType {
        mul_high(&self.lhs.operate(pos), &self.rhs.operate(pos))
    }
}
