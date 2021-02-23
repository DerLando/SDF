use sdf_vecs::{Vec3, VecType, ops::{Length, min_high, mul_high}};

mod sdf;
mod node;
mod constant;
mod variable;
mod ops;

pub use {sdf::SdfTree};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let tree = SdfTree::circle(1.0);

        assert_eq!(-1.0, tree.sign_at(&Vec3::default()));
        assert_eq!(0.0, tree.sign_at(&Vec3::new(1.0, 0.0, 0.0)));
        assert_eq!(1.0, tree.sign_at(&Vec3::new(2.0, 0.0, 0.0)));

        let other = SdfTree::circle(2.0);

        let union = SdfTree::union(tree, other);

        assert_eq!(0.0, union.sign_at(&Vec3::new(2.0, 0.0, 0.0)));

        let scale = SdfTree::circle(1.0);
        let scale = SdfTree::scale(scale, 2.0);
        assert_eq!(0.0, scale.sign_at(&Vec3::new(2.0, 0.0, 0.0)));
        let scale = SdfTree::scale(scale, 0.5);
        assert_eq!(0.0, scale.sign_at(&Vec3::new(1.0, 0.0, 0.0)));

    }
}
