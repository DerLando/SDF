use crate::{ComponentDimensionVec, Dimension};


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec1 {
    pub(crate) components: [f32; 1]
}

impl Vec1 {
    #[inline]
    pub const fn new(x: f32) -> Self {
        Self {
            components: [x]
        }
    }
}

impl Dimension for Vec1 {
    fn dimension(&self) -> u8 {
        1
    }
}

impl ComponentDimensionVec for Vec1 {}