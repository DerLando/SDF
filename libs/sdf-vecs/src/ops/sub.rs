use crate::{vec_type::VecType};

use super::{add::{add_high, add_low}};

impl std::ops::Sub<VecType> for VecType {
    type Output = VecType;

    fn sub(self, rhs: VecType) -> Self::Output {
        sub_high(&self, &rhs)
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
    use glam::{Vec2, Vec4};

    use super::*;

    #[test]
    fn it_works() {
        let lhs: VecType = Vec2::new(3.0, -4.0).into();
        let rhs: VecType = Vec4::new(1.0, -2.0, 5.0, 3.0).into();

        assert_eq!(VecType::Vec2(Vec2::new(2.0, -2.0)), sub_low(&lhs, &rhs));
        assert_eq!(VecType::Vec4(Vec4::new(2.0, -2.0, -2.0, -7.0)), sub_high(&lhs, &rhs));
    }
}