use crate::{component_access::ComponentAccess, vec_1::Vec1, vec_2::Vec2, vec_3::Vec3, vec_4::Vec4, vec_type::VecType};

trait Length {
    fn length(&self) -> f32;
}

impl Length for Vec1 {
    fn length(&self) -> f32 {
        self.x()
    }
}

impl Length for Vec2 {
    fn length(&self) -> f32 {
        (self.x() * self.x() + self.y() * self.y()).sqrt()
    }
}

impl Length for Vec3 {
    fn length(&self) -> f32 {
        (self.x() * self.x() + self.y() * self.y() + self.z() + self.z()).sqrt()
    }
}

impl Length for Vec4 {
    fn length(&self) -> f32 {
        (self.x() * self.x() + self.y() * self.y() + self.z() + self.z() + self.w() * self.w()).sqrt()
    }
}

impl Length for VecType {
    fn length(&self) -> f32 {
        match self {
            VecType::Vec1(v) => v.length(),
            VecType::Vec2(v) => v.length(),
            VecType::Vec3(v) => v.length(),
            VecType::Vec4(v) => v.length(),
        }
    }
}
