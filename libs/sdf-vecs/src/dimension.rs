use crate::vec_type::VecType;

/// A n-dimensional vector, maximum of n is 4
pub trait Dimension {
    fn dimension(&self) -> u8;
}

impl Dimension for VecType {
    fn dimension(&self) -> u8 {
        match self {
            VecType::Vec1(_) => 1,
            VecType::Vec2(_) => 2,
            VecType::Vec3(_) => 3,
            VecType::Vec4(_) => 4,
        }
    }
}