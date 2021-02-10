use sdf_vecs::{ComponentAccess, Vec1, VecType};

use crate::{Operator, Spatial, VariableContainer};

pub(crate) struct X<S: Spatial>(pub(crate) S);

impl<S> Spatial for X<S> where S: Spatial {}

impl<S> Operator<VecType> for X<S> where S: Spatial {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().x()).into()
    }
}

impl<S> VariableContainer for X<S> where S: Spatial {
    fn replace_variable(&mut self, var: &sdf_vecs::Vec3) {
        self.0.replace_variable(var)
    }
}

pub(crate) struct Y<S: Spatial>(pub(crate) S);

impl<S> Spatial for Y<S> where S: Spatial {}

impl<S> Operator<VecType> for Y<S> where S: Spatial {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().y()).into()
    }
}

impl<S> VariableContainer for Y<S> where S: Spatial {
    fn replace_variable(&mut self, var: &sdf_vecs::Vec3) {
        self.0.replace_variable(var)
    }
}

pub(crate) struct Z<S: Spatial>(pub(crate) S);

impl<S> Spatial for Z<S> where S: Spatial {}

impl<S> Operator<VecType> for Z<S> where S: Spatial {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().z()).into()
    }
}

impl<S> VariableContainer for Z<S> where S: Spatial {
    fn replace_variable(&mut self, var: &sdf_vecs::Vec3) {
        self.0.replace_variable(var)
    }
}

pub(crate) struct W<S: Spatial>(pub(crate) S);

impl<S> Spatial for W<S> where S: Spatial {}

impl<S> Operator<VecType> for W<S> where S: Spatial {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().w()).into()
    }
}

impl<S> VariableContainer for W<S> where S: Spatial {
    fn replace_variable(&mut self, var: &sdf_vecs::Vec3) {
        self.0.replace_variable(var)
    }
}