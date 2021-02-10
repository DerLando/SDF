use sdf_vecs::{Vec1, Vec3, VecType, ops::Length as lengthTrait};

use crate::{Operator, Spatial, VariableContainer, };

use super::NoOp;

impl_unary_op!(Length, length);

impl Operator<VecType> for Length {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().length()).into()
    }
}
