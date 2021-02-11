use sdf_vecs::{Vec3, VecType};

use crate::{Operator, Spatial};

impl_unary_op!(Neg, neg);

impl Operator for Neg {
    fn operate(&self, pos: &Vec3) -> VecType {
        -self.0.operate(pos)
    }
}