use crate::{ComponentDimensionVec, Dimension};


#[derive(Debug, PartialEq)]
pub struct Vec3 {
    pub(crate) components: [f32; 3]
}

impl Vec3 {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            components: [x, y, z]
        }
    }
}

impl Dimension for Vec3 {
    fn dimension(&self) -> u8 {
        3
    }
}

impl ComponentDimensionVec for Vec3 {}