use sdf_vecs::{VecType, ops::Abs as AbsTrait};

use crate::{Operator, Spatial, VariableContainer};

pub(crate) struct Abs<S: Spatial>(pub(crate) S);

impl<S> Spatial for Abs<S> where S: Spatial { }

impl<S> Operator<VecType> for Abs<S> where S: Spatial {
    fn operate(&self) -> VecType {
        self.0.operate().abs()
    }
}

impl<S> VariableContainer for Abs<S> where S: Spatial {
    fn replace_variable(&mut self, var: &sdf_vecs::Vec3) {
        self.0.replace_variable(var)
    }
}