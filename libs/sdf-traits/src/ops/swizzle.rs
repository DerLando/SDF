use sdf_vecs::{ComponentAccess, Vec1, VecType, Vec3};

use crate::{Operator, Spatial, VariableContainer};

impl_unary_op!(X);

impl Operator<VecType> for X {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().x()).into()
    }
}

impl_unary_op!(Y);

impl Operator<VecType> for Y {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().y()).into()
    }
}

impl_unary_op!(Z);

impl Operator<VecType> for Z {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().z()).into()
    }
}

impl_unary_op!(W);

impl Operator<VecType> for W {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().w()).into()
    }
}
