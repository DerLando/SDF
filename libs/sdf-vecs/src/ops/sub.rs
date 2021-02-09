use crate::{vec_1::Vec1, vec_2::Vec2, vec_3::Vec3, vec_4::Vec4, vec_type::VecType};

use super::{OperatorKind, add::{add_high, add_low}};



impl std::ops::Sub<Vec1> for Vec1 {
    type Output = Vec1;

    fn sub(self, rhs: Vec1) -> Self::Output {
        self + -rhs
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        self + -rhs
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self + -rhs
    }
}

impl std::ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        self + -rhs
    }
}

pub fn sub_high(a: &VecType, b: &VecType) -> VecType {
    add_high(a, &-*b)
}

pub fn sub_low(a: &VecType, b: &VecType) -> VecType {
    add_low(a, &-*b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lhs: VecType = Vec2::new(3.0, -4.0).into();
        let rhs: VecType = Vec4::new(1.0, -2.0, 5.0, 3.0).into();

        assert_eq!(VecType::Vec2(Vec2::new(2.0, -2.0)), sub_low(&lhs, &rhs));
        assert_eq!(VecType::Vec4(Vec4::new(2.0, -2.0, -2.0, -7.0)), sub_high(&lhs, &rhs));
    }
}