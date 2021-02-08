use crate::{ComponentDimensionVec, Dimension};


#[derive(Debug, PartialEq)]
pub struct Vec4 {
    pub(crate) components: [f32; 4]
}

impl Vec4 {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            components: [x, y, z, w]
        }
    }
}

impl Dimension for Vec4 {
    fn dimension(&self) -> u8 {
        4
    }
}

impl ComponentDimensionVec for Vec4 {}