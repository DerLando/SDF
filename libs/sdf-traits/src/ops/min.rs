use sdf_vecs::{VecType, ops::min_high, Vec3};

use crate::{Operator, Spatial};

impl_binary_op!(Min, min);

impl Operator for Min {
    fn operate(&self, pos: &Vec3) -> VecType {
        min_high(&self.lhs.operate(pos), &self.rhs.operate(pos))
    }
}
