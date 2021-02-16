use crate::{scale::{Scale}, vec_type::VecType};

use super::{OperatorKind, scale_same};

impl std::ops::Add<VecType> for VecType {
    type Output = VecType;

    fn add(self, rhs: VecType) -> Self::Output {
        add_high(&self, &rhs)
    }
}

fn add(a: &VecType, b: &VecType, kind: OperatorKind) -> VecType {
    let (dim, lhs, rhs) = scale_same(a, b, kind);

    match dim {
        1 => (lhs.scale1() + rhs.scale1()).into(),
        2 => (lhs.scale2() + rhs.scale2()).into(),
        3 => (lhs.scale3() + rhs.scale3()).into(),
        4 => (lhs.scale4() + rhs.scale4()).into(),
        _ => unreachable!()
    }
}

pub fn add_high(a: &VecType, b: &VecType) -> VecType {
    add(a, b, OperatorKind::High)
}

pub fn add_low(a: &VecType, b: &VecType) -> VecType {
    add(a, b, OperatorKind::Low)
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::*;

    #[test]
    fn it_works() {
        let lhs: VecType = Vec3::new(1.0, -3.0, 4.0).into();
        let rhs: VecType = 2.0.into();

        assert_eq!(VecType::Scalar(3.0), add_low(&lhs, &rhs));
        assert_eq!(VecType::Vec3(Vec3::new(3.0, -1.0, 6.0)), add_high(&lhs, &rhs));
    }
}