use sdf_vecs::{Vec1, Vec3, VecType, ops::Length};

use crate::{Operator, Spatial, VariableContainer};

pub(crate) struct LengthOp<S: Spatial>(pub(crate) S);

impl<S> Spatial for LengthOp<S> where S: Spatial { }

impl<S> VariableContainer for LengthOp<S>
where S: Spatial {
    fn replace_variable(&mut self, var: &Vec3) {
        self.0.replace_variable(var);
    }
}

impl<S> Operator<VecType> for LengthOp<S>
where S: Spatial {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().length()).into()
    }
}