use sdf_vecs::{Vec3, VecType};

use crate::{Operator, Spatial, VariableContainer};

pub(crate) struct NegOp<S: Spatial>(pub(crate) S);

impl<S> Spatial for NegOp<S> where S: Spatial { }

impl<S> VariableContainer for NegOp<S> where S: Spatial {
    fn replace_variable(&mut self, var: &Vec3) {
        self.0.replace_variable(var)
    }
}

impl<S> Operator<VecType> for NegOp<S> where S: Spatial {
    fn operate(&self) -> VecType {
        -self.0.operate()
    }
}