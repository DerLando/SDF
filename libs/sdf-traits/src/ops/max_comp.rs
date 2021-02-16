use glam::Vec3;
use sdf_vecs::{VecType, ops::MaxComp as MaxCompTrait};

use crate::{Operator, Spatial, };

impl_unary_op!(MaxComp, max_comp);

impl Operator for MaxComp {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).max_comp().into()
    }
}