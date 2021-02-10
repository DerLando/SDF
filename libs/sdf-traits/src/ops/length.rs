use sdf_vecs::{Vec1, Vec3, VecType, ops::Length as lengthTrait};

use crate::{Operator, Spatial, VariableContainer, Boxed};

impl_unary_op!(Length);

impl Operator<VecType> for Length {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().length()).into()
    }
}