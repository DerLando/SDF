use sdf_vecs::{VecType, ops::max_high};

use crate::{Operator, Spatial, VariableContainer};

#[derive(Clone)]
pub(crate) struct Max<L, R>
where L: Spatial, R: Spatial
{
    pub(crate) lhs: L,
    pub(crate) rhs: R
}

impl<L, R> Spatial for Max<L, R>
where L: Spatial, R: Spatial
{ }

impl<L, R> Operator<VecType> for Max<L, R>
where L: Spatial, R: Spatial
{
    fn operate(&self) -> VecType {
        max_high(&self.lhs.operate(), &self.rhs.operate())
    }
}

impl<L, R> VariableContainer for Max<L, R>
where L: Spatial, R: Spatial
{
    fn replace_variable(&mut self, var: &sdf_vecs::Vec3) {
        self.lhs.replace_variable(var);
        self.rhs.replace_variable(var);
    }
}
