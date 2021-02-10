use sdf_vecs::{Vec1, Vec3, VecType, ops::MaxComp as MaxCompTrait};

use crate::{Operator, Spatial, VariableContainer};

#[derive(Clone)]
pub(crate) struct MaxComp<S: Spatial>(pub(crate) S);

impl<S> Spatial for MaxComp<S> where S: Spatial { }

impl<S> VariableContainer for MaxComp<S> where S: Spatial {
    fn replace_variable(&mut self, var: &Vec3) {
        self.0.replace_variable(var)
    }
}

impl<S> Operator<VecType> for MaxComp<S> where S: Spatial {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().max_comp()).into()
    }
}