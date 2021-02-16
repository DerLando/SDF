use dimension::Dimension;

mod component_access;
mod swizzle;
mod scale;
pub mod ops;
mod dimension;
mod vec_type;

trait ComponentDimensionVec: component_access::ComponentAccess + Dimension {}

pub use self::{vec_type::VecType,
    component_access::ComponentAccess, swizzle::{SwizzleDim, SwizzleDim2, SwizzleDim3}
};
pub use glam::Vec3;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
