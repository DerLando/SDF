use sdf_vecs::{Vec1, Vec3, VecType, ops::MaxComp as MaxCompTrait};

use crate::{Operator, Spatial, VariableContainer, Boxed};

impl_unary_op!(MaxComp);

impl Operator<VecType> for MaxComp {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().max_comp()).into()
    }
}