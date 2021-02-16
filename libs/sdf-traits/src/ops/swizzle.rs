use glam::Vec3;
use sdf_vecs::{ComponentAccess, SwizzleDim2, VecType};

use crate::{Operator, Spatial, VariableContainer, };

impl_unary_op!(X, x);

impl Operator for X {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).x().into()
    }
}

impl_unary_op!(Y, y);

impl Operator for Y {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).y().into()
    }
}

impl_unary_op!(Z, z);

impl Operator for Z {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).z().into()
    }
}

impl_unary_op!(W, w);

impl Operator for W {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).w().into()
    }
}

impl_unary_op!(XX, xx);

impl Operator for XX {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).xx().into()
    }
}

impl_unary_op!(XY, xy);

impl Operator for XY {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).xy().into()
    }
}

impl_unary_op!(XZ, xz);

impl Operator for XZ {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).xz().into()
    }
}

impl_unary_op!(YY, yy);

impl Operator for YY {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).yy().into()
    }
}

impl_unary_op!(YX, yx);

impl Operator for YX {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).yx().into()
    }
}

impl_unary_op!(YZ, yz);

impl Operator for YZ {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).yz().into()
    }
}

impl_unary_op!(ZX, zx);

impl Operator for ZX {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).zx().into()
    }
}

impl_unary_op!(ZY, zy);

impl Operator for ZY {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).zy().into()
    }
}

impl_unary_op!(ZZ, zz);

impl Operator for ZZ {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).zz().into()
    }
}
