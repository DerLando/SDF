use sdf_vecs::{VecType, ops::max_high, Vec3};

use crate::{Operator, Spatial, VariableContainer};

impl_binary_op!(Max);

impl Operator<VecType> for Max {
    fn operate(&self) -> VecType {
        max_high(&self.lhs.operate(), &self.rhs.operate())
    }
}
