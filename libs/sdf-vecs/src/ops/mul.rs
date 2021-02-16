use crate::{VecType, scale::Scale};

use super::{OperatorKind, scale_same};

impl std::ops::Mul<VecType> for VecType {
    type Output = VecType;

    fn mul(self, rhs: VecType) -> Self::Output {
        mul_high(&self, &rhs)
    }
}

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
