use component_access::ComponentAccess;
use dimension::Dimension;

mod vec_1;
mod vec_2;
mod vec_3;
mod vec_4;
mod component_access;
mod swizzle;
mod scale;
mod ops;
mod dimension;
mod vec_type;

pub trait ComponentDimensionVec: ComponentAccess + Dimension {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
