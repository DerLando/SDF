use crate::{VecType, scale::Scale};

use super::{OperatorKind, scale_same};

trait Min<T> {
    fn min(&self, other: &T) -> Self;
}

fn min(a: &VecType, b: &VecType, kind: OperatorKind) -> VecType {
    let (dim, lhs, rhs) = scale_same(a, b, kind);

    match dim {
        1 => VecType::Scalar(lhs.scale1().min(rhs.scale1())),
        2 => VecType::Vec2(lhs.scale2().min(rhs.scale2())),
        3 => VecType::Vec3(lhs.scale3().min(rhs.scale3())),
        4 => VecType::Vec4(lhs.scale4().min(rhs.scale4())),
        _ => unreachable!()
    }
}

pub fn min_high(a: &VecType, b: &VecType) -> VecType {
    min(a, b, OperatorKind::High)
}

pub fn min_low(a: &VecType, b: &VecType) -> VecType {
    min(a, b, OperatorKind::Low)
}

pub trait MinComp {
    fn min_comp(&self) -> f32;
}

impl MinComp for VecType {
    fn min_comp(&self) -> f32 {
        match self {
            VecType::Scalar(s) => *s,
            VecType::Vec2(v) => v.min_element(),
            VecType::Vec3(v) => v.min_element(),
            VecType::Vec4(v) => v.min_element(),
        }
    }
}