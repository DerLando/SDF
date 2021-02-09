use crate::{ComponentAccess, Vec1, Vec2, Vec3, Vec4, VecType, scale::Scale};

use super::{OperatorKind, scale_same};

trait Max<T> {
    fn max(&self, other: &T) -> Self;
}

impl Max<Vec1> for Vec1 {
    fn max(&self, other: &Vec1) -> Self {
        Vec1::new(self.x().max(other.x()))
    }
}

impl Max<Vec2> for Vec2 {
    fn max(&self, other: &Vec2) -> Self {
        Vec2::new(self.x().max(other.x()), self.y().max(other.y()))
    }
}

impl Max<Vec3> for Vec3 {
    fn max(&self, other: &Vec3) -> Self {
        Vec3::new(self.x().max(other.x()), self.y().max(other.y()), self.z().max(other.z()))
    }
}

impl Max<Vec4> for Vec4 {
    fn max(&self, other: &Vec4) -> Self {
        Vec4::new(
            self.x().max(other.x()),
            self.y().max(other.y()),
            self.z().max(other.z()),
            self.w().max(other.w())
        )
    }
}

fn max(a: &VecType, b: &VecType, kind: OperatorKind) -> VecType {
    let (dim, lhs, rhs) = scale_same(a, b, kind);

    match dim {
        1 => VecType::Vec1(lhs.scale1().max(&rhs.scale1())),
        2 => VecType::Vec2(lhs.scale2().max(&rhs.scale2())),
        3 => VecType::Vec3(lhs.scale3().max(&rhs.scale3())),
        4 => VecType::Vec4(lhs.scale4().max(&rhs.scale4())),
        _ => unreachable!()
    }
}

pub fn max_high(a: &VecType, b: &VecType) -> VecType {
    max(a, b, OperatorKind::High)
}

pub fn max_low(a: &VecType, b: &VecType) -> VecType {
    max(a, b, OperatorKind::Low)
}