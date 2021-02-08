use crate::{component_access::ComponentAccess, vec_1::Vec1, vec_2::Vec2, vec_3::Vec3, vec_4::Vec4, vec_type::VecType};

impl std::ops::Neg for Vec1 {
    type Output = Vec1;

    fn neg(self) -> Self::Output {
        Self::new(-self.x())
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y())
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl std::ops::Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z(), -self.w())
    }
}

impl std::ops::Neg for VecType {
    type Output = VecType;

    fn neg(self) -> Self::Output {
        match self {
            VecType::Vec1(v) => VecType::Vec1(-v),
            VecType::Vec2(v) => VecType::Vec2(-v),
            VecType::Vec3(v) => VecType::Vec3(-v),
            VecType::Vec4(v) => VecType::Vec4(-v),
        }
    }
}