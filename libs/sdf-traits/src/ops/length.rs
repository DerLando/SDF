use sdf_vecs::{Vec1, Vec3, VecType, ops::Length as lengthTrait};

use crate::{Operator, Spatial};

impl_unary_op!(Length, length);

impl Operator for Length {
    fn operate(&self, pos: &Vec3) -> VecType {
        Vec1::new(self.0.operate(pos).length()).into()
    }
}
