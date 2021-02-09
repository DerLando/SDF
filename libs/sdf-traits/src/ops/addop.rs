use sdf_vecs::{Vec3, VecType, ops::add_high};

use crate::{Operator, Spatial, VariableContainer};

pub(crate)struct AddOp<L, R> where L: Spatial, R: Spatial{
    pub(crate) lhs: L,
    pub(crate) rhs: R
}

impl<L, R> Spatial for AddOp<L, R> where L: Spatial, R: Spatial {}

impl<L, R> VariableContainer for AddOp<L, R> where L: Spatial, R: Spatial {
    fn replace_variable(&mut self, var: &Vec3) {
        self.lhs.replace_variable(var);
        self.rhs.replace_variable(var);
    }
}

impl<L, R> Operator<VecType> for AddOp<L, R> where L: Spatial, R: Spatial {
    fn operate(&self) -> VecType {
        add_high(&self.lhs.operate(), &self.rhs.operate())
    }
}
