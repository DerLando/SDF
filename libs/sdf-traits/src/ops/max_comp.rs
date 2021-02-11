use sdf_vecs::{Vec1, Vec3, VecType, ops::MaxComp as MaxCompTrait};

use crate::{Operator, Spatial, };

impl_unary_op!(MaxComp, max_comp);

impl Operator for MaxComp {
    fn operate(&self, pos: &Vec3) -> VecType {
        Vec1::new(self.0.operate(pos).max_comp()).into()
    }
}