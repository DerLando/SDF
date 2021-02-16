use crate::VecType;

/// Access components (e.g.: x, y, z, w) of an Vec.
/// If components of lower dimensional vecs are accessed, they will internally wrap around.
pub trait ComponentAccess {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
    fn w(&self) -> f32;
}

impl ComponentAccess for VecType {
    fn x(&self) -> f32 {
        match self {
            VecType::Scalar(s) => *s,
            VecType::Vec2(v) => v.x,
            VecType::Vec3(v) => v.x,
            VecType::Vec4(v) => v.x,
        }
    }

    fn y(&self) -> f32 {
        match self {
            VecType::Scalar(s) => *s,
            VecType::Vec2(v) => v.y,
            VecType::Vec3(v) => v.y,
            VecType::Vec4(v) => v.y,
        }
    }

    fn z(&self) -> f32 {
        match self {
            VecType::Scalar(s) => *s,
            VecType::Vec2(v) => v.x, // wrap around
            VecType::Vec3(v) => v.z,
            VecType::Vec4(v) => v.z,
        }
    }

    fn w(&self) -> f32 {
        match self {
            VecType::Scalar(s) => *s,
            VecType::Vec2(v) => v.y,
            VecType::Vec3(v) => v.x,
            VecType::Vec4(v) => v.w,
        }
    }
}
