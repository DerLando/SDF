use crate::VecType;

impl std::ops::Neg for VecType {
    type Output = VecType;

    fn neg(self) -> Self::Output {
        match self {
            VecType::Scalar(s) => VecType::Scalar(-s),
            VecType::Vec2(v) => VecType::Vec2(-v),
            VecType::Vec3(v) => VecType::Vec3(-v),
            VecType::Vec4(v) => VecType::Vec4(-v),
        }
    }
}