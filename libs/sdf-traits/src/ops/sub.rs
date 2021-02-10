use sdf_vecs::{VecType, ops::sub_high, Vec3};

use crate::{Operator, Spatial, VariableContainer};

impl_binary_op!(Sub);

impl Operator<VecType> for Sub {
    fn operate(&self) -> VecType {
        sub_high(&self.lhs.operate(), &self.rhs.operate())
    }
}
