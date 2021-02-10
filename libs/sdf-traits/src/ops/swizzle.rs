use sdf_vecs::{ComponentAccess, SwizzleDim2, Vec1, Vec2, Vec3, VecType};

use crate::{Operator, Spatial, VariableContainer, };

impl_unary_op!(X, x);

impl Operator<VecType> for X {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().x()).into()
    }
}

impl_unary_op!(Y, y);

impl Operator<VecType> for Y {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().y()).into()
    }
}

impl_unary_op!(Z, z);

impl Operator<VecType> for Z {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().z()).into()
    }
}

impl_unary_op!(W, w);

impl Operator<VecType> for W {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().w()).into()
    }
}

impl_unary_op!(XX, xx);

impl Operator<VecType> for XX {
    fn operate(&self) -> VecType {
        self.0.operate().xx().into()
    }
}

impl_unary_op!(XY, xy);

impl Operator<VecType> for XY {
    fn operate(&self) -> VecType {
        self.0.operate().xy().into()
    }
}

impl_unary_op!(YY, yy);

impl Operator<VecType> for YY {
    fn operate(&self) -> VecType {
        self.0.operate().yy().into()
    }
}

impl_unary_op!(YX, yx);

impl Operator<VecType> for YX {
    fn operate(&self) -> VecType {
        self.0.operate().yx().into()
    }
}
