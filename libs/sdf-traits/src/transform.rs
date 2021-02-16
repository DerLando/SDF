use glam::{Mat4, Vec3};

// TODO: Need a way to express scale

pub(crate) type Transform = Mat4;

pub(crate) struct TransformHelper;


/// Create transforms
impl TransformHelper {
    pub(crate) fn identity() -> Transform {
        Mat4::default()
    }

    pub(crate) fn translation(v: &Vec3) -> Transform {
        Mat4::from_translation(*v)
    }
}

/// Use transforms
impl TransformHelper {
    pub(crate) fn transform(transform: &Transform, v: &Vec3) -> Vec3 {
        transform.transform_vector3(*v)
    }
}

#[cfg(test)]
mod test {
    use crate::SDFTree;

    use super::*;

    #[test]
    fn zero_translation_does_nothing() {
        // base point and sanity check
        let mut point = SDFTree::point();
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
        let mut point = SDFTree::point();
        let x_dist = 0.42;
        assert!((x_dist - point.sign_at(&Vec3::new(x_dist, 0.0, 0.0))).abs() < std::f32::EPSILON);

        // translate along x
        point.translate(&Vec3::new(x_dist, 0.0, 0.0));
        assert_eq!(0.0, point.sign_at(&Vec3::new(x_dist, 0.0, 0.0)));

        // translate along negative x, should be at origin again
        point.translate(&Vec3::new(-x_dist, 0.0, 0.0));
        assert_eq!(0.0, point.sign_at(&Vec3::default()));
    }
}
