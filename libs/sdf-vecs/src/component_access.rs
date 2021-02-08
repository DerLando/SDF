use crate::{vec_1::Vec1, vec_2::Vec2, vec_3::Vec3, vec_4::Vec4, vec_type::VecType};


/// Access components (e.g.: x, y, z, w) of an Vec.
/// If components of lower dimensional vecs are accessed, they will internally wrap around.
pub trait ComponentAccess {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
    fn w(&self) -> f32;
}

impl ComponentAccess for Vec1 {
    fn x(&self) -> f32 {
        self.components[0]
    }

    fn y(&self) -> f32 {
        self.components[0]
    }

    fn z(&self) -> f32 {
        self.components[0]
    }

    fn w(&self) -> f32 {
        self.components[0]
    }
}

impl ComponentAccess for Vec2 {
    fn x(&self) -> f32 {
        self.components[0]
    }

    fn y(&self) -> f32 {
        self.components[1]
    }

    fn z(&self) -> f32 {
        self.components[0]
    }

    fn w(&self) -> f32 {
        self.components[1]
    }
}

impl ComponentAccess for Vec3 {
    fn x(&self) -> f32 {
        self.components[0]
    }

    fn y(&self) -> f32 {
        self.components[1]
    }

    fn z(&self) -> f32 {
        self.components[2]
    }

    fn w(&self) -> f32 {
        self.components[0]
    }
}

impl ComponentAccess for Vec4 {
    fn x(&self) -> f32 {
        self.components[0]
    }

    fn y(&self) -> f32 {
        self.components[1]
    }

    fn z(&self) -> f32 {
        self.components[2]
    }

    fn w(&self) -> f32 {
        self.components[3]
    }
}

impl ComponentAccess for VecType {
    fn x(&self) -> f32 {
        match self {
            VecType::Vec1(v) => v.x(),
            VecType::Vec2(v) => v.x(),
            VecType::Vec3(v) => v.x(),
            VecType::Vec4(v) => v.x(),
        }
    }

    fn y(&self) -> f32 {
        match self {
            VecType::Vec1(v) => v.y(),
            VecType::Vec2(v) => v.y(),
            VecType::Vec3(v) => v.y(),
            VecType::Vec4(v) => v.y(),
        }
    }

    fn z(&self) -> f32 {
        match self {
            VecType::Vec1(v) => v.z(),
            VecType::Vec2(v) => v.z(),
            VecType::Vec3(v) => v.z(),
            VecType::Vec4(v) => v.z(),
        }
    }

    fn w(&self) -> f32 {
        match self {
            VecType::Vec1(v) => v.z(),
            VecType::Vec2(v) => v.z(),
            VecType::Vec3(v) => v.z(),
            VecType::Vec4(v) => v.z(),
        }
    }
}
