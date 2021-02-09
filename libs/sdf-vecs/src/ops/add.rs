use crate::{Dimension, component_access::ComponentAccess, scale::{Scale, scale_n}, vec_1::Vec1, vec_2::Vec2, vec_3::Vec3, vec_4::Vec4, vec_type::VecType};

use super::{OperatorKind, scale_same};

impl std::ops::Add<Vec1> for Vec1 {
    type Output = Vec1;

    fn add(self, rhs: Vec1) -> Self::Output {
        Self::new(self.x() + rhs.x())
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl std::ops::Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z(), self.w() + rhs.w())
    }
}


// impl std::ops::Add<VecType> for VecType {
//     type Output = VecType;

//     fn add(self, rhs: VecType) -> Self::Output {
//         add_high(&self, &rhs)
//     }
// }

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
    use super::*;

    #[test]
    fn it_works() {
        let lhs: VecType = Vec3::new(1.0, -3.0, 4.0).into();
        let rhs: VecType = Vec1::new(2.0).into();

        assert_eq!(VecType::Vec1(Vec1::new(3.0)), add_low(&lhs, &rhs));
        assert_eq!(VecType::Vec3(Vec3::new(3.0, -1.0, 6.0)), add_high(&lhs, &rhs));
    }
}