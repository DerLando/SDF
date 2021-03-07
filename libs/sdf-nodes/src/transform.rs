#[cfg(test)]
mod test {
    use crate::SdfTree;
    use sdf_vecs::*;
    use super::*;

    #[test]
    fn zero_translation_does_nothing() {
        // base point and sanity check
        let mut point = SdfTree::point();
        assert_eq!(0.0, point.sign_at(&Vec3::default()));
        assert_eq!(0.3, point.sign_at(&Vec3::new(0.3, 0.0, 0.0)));

        // translate and see what happens
        point.translate(&Vec3::default());
        assert_eq!(0.0, point.sign_at(&Vec3::default()));
        assert_eq!(0.3, point.sign_at(&Vec3::new(0.3, 0.0, 0.0)));
    }

    #[test]
    fn translation_is_commutative() {
        // base point and sanity check
        let mut point = SdfTree::point();
        let x_dist = 0.42;
        assert!((x_dist - point.sign_at(&Vec3::new(x_dist, 0.0, 0.0))).abs() < std::f32::EPSILON);

        // translate along x
        point.translate(&Vec3::new(x_dist, 0.0, 0.0));
        assert_eq!(0.0, point.sign_at(&Vec3::new(x_dist, 0.0, 0.0)));

        // translate along negative x, should be at origin again
        point.translate(&Vec3::new(-x_dist, 0.0, 0.0));
        assert_eq!(0.0, point.sign_at(&Vec3::default()));
    }

    #[test]
    fn unit_scale_does_nothing() {
        // base sphere
        let mut sphere = SdfTree::sphere(1.0);
        assert_eq!(0.0, sphere.sign_at(&Vec3::new(1.0, 0.0, 0.0)));

        // scale
        sphere.scale(1.0);
        assert_eq!(0.0, sphere.sign_at(&Vec3::new(1.0, 0.0, 0.0)));
    }

    #[test]
    fn scale_is_commutative() {
        // base sphere
        let mut sphere = SdfTree::sphere(1.0);
        assert_eq!(0.0, sphere.sign_at(&Vec3::new(1.0, 0.0, 0.0)));

        // scale
        sphere.scale(1.5);
        assert_eq!(0.0, sphere.sign_at(&Vec3::new(1.5, 0.0, 0.0)));

        // scale again
        sphere.scale(1.5);
        assert_eq!(0.0, sphere.sign_at(&Vec3::new(2.25, 0.0, 0.0)));

        // new sphere
        let mut sphere = SdfTree::sphere(1.0);
        sphere.scale(2.25);
        assert_eq!(0.0, sphere.sign_at(&Vec3::new(2.25, 0.0, 0.0)));
    }}
