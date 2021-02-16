use crate::{VecType, scale::Scale};

use super::{OperatorKind, scale_same};

trait Max<T> {
    fn max(&self, other: &T) -> Self;
}

fn max(a: &VecType, b: &VecType, kind: OperatorKind) -> VecType {
    let (dim, lhs, rhs) = scale_same(a, b, kind);

    match dim {
        1 => VecType::Scalar(lhs.scale1().max(rhs.scale1())),
        2 => VecType::Vec2(lhs.scale2().max(rhs.scale2())),
        3 => VecType::Vec3(lhs.scale3().max(rhs.scale3())),
        4 => VecType::Vec4(lhs.scale4().max(rhs.scale4())),
        _ => unreachable!()
    }
}

pub fn max_high(a: &VecType, b: &VecType) -> VecType {
    max(a, b, OperatorKind::High)
}

pub fn max_low(a: &VecType, b: &VecType) -> VecType {
    max(a, b, OperatorKind::Low)
}

pub trait MaxComp {
    fn max_comp(&self) -> f32;
}

impl MaxComp for VecType {
    fn max_comp(&self) -> f32 {
        match self {
            VecType::Scalar(s) => *s,
            VecType::Vec2(v) => v.max_element(),
            VecType::Vec3(v) => v.max_element(),
            VecType::Vec4(v) => v.max_element(),
        }
    }
}