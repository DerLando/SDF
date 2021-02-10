use crate::{Dimension, component_access::ComponentAccess, scale::{Scale, scale_n}, vec_1::Vec1, vec_2::Vec2, vec_3::Vec3, vec_4::Vec4, vec_type::VecType};

use super::{OperatorKind, scale_same};

impl std::ops::Mul<Vec1> for Vec1 {
    type Output = Vec1;

    fn mul(self, rhs: Vec1) -> Self::Output {
        Self::new(self.x() * rhs.x())
    }
}

impl std::ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x() * rhs.x(), self.y() * rhs.y())
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl std::ops::Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Self::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z(), self.w() * rhs.w())
    }
}


// impl std::ops::Mul<VecType> for VecType {
//     type Output = VecType;

//     fn Mul(self, rhs: VecType) -> Self::Output {
//         Mul_high(&self, &rhs)
//     }
// }

fn mul(a: &VecType, b: &VecType, kind: OperatorKind) -> VecType {
    let (dim, lhs, rhs) = scale_same(a, b, kind);

    match dim {
        1 => (lhs.scale1() * rhs.scale1()).into(),
        2 => (lhs.scale2() * rhs.scale2()).into(),
        3 => (lhs.scale3() * rhs.scale3()).into(),
        4 => (lhs.scale4() * rhs.scale4()).into(),
        _ => unreachable!()
    }
}

pub fn mul_high(a: &VecType, b: &VecType) -> VecType {
    mul(a, b, OperatorKind::High)
}

pub fn mul_low(a: &VecType, b: &VecType) -> VecType {
    mul(a, b, OperatorKind::Low)
}
