use sdf_vecs::{VecType, ops::min_high, Vec3};

use crate::{Operator, Spatial, VariableContainer};

impl_binary_op!(Min, min);

impl Operator<VecType> for Min {
    fn operate(&self) -> VecType {
        min_high(&self.lhs.operate(), &self.rhs.operate())
    }
}
