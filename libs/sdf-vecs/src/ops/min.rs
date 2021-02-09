use crate::{ComponentAccess, Vec1, Vec2, Vec3, Vec4, VecType, scale::Scale};

use super::{OperatorKind, scale_same};

trait Min<T> {
    fn min(&self, other: &T) -> Self;
}

impl Min<Vec1> for Vec1 {
    fn min(&self, other: &Vec1) -> Self {
        Vec1::new(self.x().min(other.x()))
    }
}

impl Min<Vec2> for Vec2 {
    fn min(&self, other: &Vec2) -> Self {
        Vec2::new(self.x().min(other.x()), self.y().min(other.y()))
    }
}

impl Min<Vec3> for Vec3 {
    fn min(&self, other: &Vec3) -> Self {
        Vec3::new(self.x().min(other.x()), self.y().min(other.y()), self.z().min(other.z()))
    }
}

impl Min<Vec4> for Vec4 {
    fn min(&self, other: &Vec4) -> Self {
        Vec4::new(
            self.x().min(other.x()),
            self.y().min(other.y()),
            self.z().min(other.z()),
            self.w().min(other.w())
        )
    }
}

fn min(a: &VecType, b: &VecType, kind: OperatorKind) -> VecType {
    let (dim, lhs, rhs) = scale_same(a, b, kind);

    match dim {
        1 => VecType::Vec1(lhs.scale1().min(&rhs.scale1())),
        2 => VecType::Vec2(lhs.scale2().min(&rhs.scale2())),
        3 => VecType::Vec3(lhs.scale3().min(&rhs.scale3())),
        4 => VecType::Vec4(lhs.scale4().min(&rhs.scale4())),
        _ => unreachable!()
    }
}

pub fn min_high(a: &VecType, b: &VecType) -> VecType {
    min(a, b, OperatorKind::High)
}

pub fn min_low(a: &VecType, b: &VecType) -> VecType {
    min(a, b, OperatorKind::Low)
}