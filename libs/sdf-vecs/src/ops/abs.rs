use crate::{ComponentAccess, Vec1, Vec2, Vec3, Vec4, VecType};

pub trait Abs {
    fn abs(&self) -> Self;
}

impl Abs for Vec1 {
    fn abs(&self) -> Self {
        Vec1::new(self.x().abs())
    }
}

impl Abs for Vec2 {
    fn abs(&self) -> Self {
        Vec2::new(self.x().abs(), self.y().abs())
    }
}

impl Abs for Vec3 {
    fn abs(&self) -> Self {
        Vec3::new(self.x().abs(), self.y().abs(), self.z().abs())
    }
}

impl Abs for Vec4 {
    fn abs(&self) -> Self {
        Vec4::new(self.x().abs(), self.y().abs(), self.z().abs(), self.w().abs())
    }
}

impl Abs for VecType {
    fn abs(&self) -> Self {
        match self {
            VecType::Vec1(v) => VecType::Vec1(v.abs()),
            VecType::Vec2(v) => VecType::Vec2(v.abs()),
            VecType::Vec3(v) => VecType::Vec3(v.abs()),
            VecType::Vec4(v) => VecType::Vec4(v.abs()),
        }
    }
}