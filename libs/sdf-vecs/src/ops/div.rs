use crate::{scale::{Scale}, vec_type::VecType};

use super::{OperatorKind, scale_same};

impl std::ops::Div<VecType> for VecType {
    type Output = VecType;

    fn div(self, rhs: VecType) -> Self::Output {
        div_high(&self, &rhs)
    }
}

fn div(a: &VecType, b: &VecType, kind: OperatorKind) -> VecType {
    let (dim, lhs, rhs) = scale_same(a, b, kind);

    match dim {
        1 => (lhs.scale1() / rhs.scale1()).into(),
        2 => (lhs.scale2() / rhs.scale2()).into(),
        3 => (lhs.scale3() / rhs.scale3()).into(),
        4 => (lhs.scale4() / rhs.scale4()).into(),
        _ => unreachable!()
    }
}

pub fn div_high(a: &VecType, b: &VecType) -> VecType {
    div(a, b, OperatorKind::High)
}

pub fn div_low(a: &VecType, b: &VecType) -> VecType {
    div(a, b, OperatorKind::Low)
}
