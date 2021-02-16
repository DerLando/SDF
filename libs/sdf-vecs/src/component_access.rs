use glam::{Vec2, Vec3, Vec4};

use crate::VecType;

/// Access components (e.g.: x, y, z, w) of an Vec.
/// If components of lower dimensional vecs are accessed, they will internally wrap around.
pub trait ComponentAccess {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
    fn w(&self) -> f32;
}

// impl ComponentAccess for Vec2 {
//     fn x(&self) -> f32 {
//         self.x
//     }

//     fn y(&self) -> f32 {
//         self.y
//     }

//     fn z(&self) -> f32 {
//         self.x
//     }

//     fn w(&self) -> f32 {
//         self.y
//     }
// }

// impl ComponentAccess for Vec3 {
//     fn x(&self) -> f32 {
//         self.x
//     }

//     fn y(&self) -> f32 {
//         self.y
//     }

//     fn z(&self) -> f32 {
//         self.z
//     }

//     fn w(&self) -> f32 {
//         self.x
//     }
// }

// impl ComponentAccess for Vec4 {
//     fn x(&self) -> f32 {
//         self.x
//     }

//     fn y(&self) -> f32 {
//         self.y
//     }

//     fn z(&self) -> f32 {
//         self.z
//     }

//     fn w(&self) -> f32 {
//         self.w
//     }
// }

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
