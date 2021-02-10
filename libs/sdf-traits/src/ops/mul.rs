use sdf_vecs::{Vec3, VecType, ops::{mul_high}};

use crate::{Operator, Spatial, VariableContainer, };

impl_binary_op!(Mul, mul);

impl Operator<VecType> for Mul {
    fn operate(&self) -> VecType {
        mul_high(&self.lhs.operate(), &self.rhs.operate())
    }
}
