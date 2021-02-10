use dimension::Dimension;

mod vec_1;
mod vec_2;
mod vec_3;
mod vec_4;
mod component_access;
mod swizzle;
mod scale;
pub mod ops;
mod dimension;
mod vec_type;

trait ComponentDimensionVec: component_access::ComponentAccess + Dimension {}

pub use self::{vec_1::Vec1, vec_2::Vec2, vec_3::Vec3, vec_4::Vec4, vec_type::VecType,
    component_access::ComponentAccess, swizzle::{SwizzleDim, SwizzleDim2, SwizzleDim3}
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
