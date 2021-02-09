use crate::{ComponentDimensionVec, Dimension};


#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vec2 {
    pub(crate) components: [f32; 2]
}

impl Vec2 {
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self {
            components: [x, y]
        }
    }
}

impl Dimension for Vec2 {
    fn dimension(&self) -> u8 {
        2
    }
}

impl ComponentDimensionVec for Vec2 {}