use glam::{Mat4, Vec3};

// TODO: Need a way to express scale

pub(crate) type Transform = Mat4;

pub(crate) struct TransformHelper;

pub enum RotationAxis {
    X,
    Y,
    Z
}


/// Create transforms
impl TransformHelper {
    pub(crate) fn identity() -> Transform {
        Mat4::default()
    }

    pub(crate) fn translation(v: &Vec3) -> Transform {
        Mat4::from_translation(*v)
    }

    pub(crate) fn rotate(angle: f32, axis: RotationAxis) -> Transform {
        match axis {
            RotationAxis::X => Self::rotate_x(angle),
            RotationAxis::Y => Self::rotate_y(angle),
            RotationAxis::Z => Self::rotate_z(angle),
        }
    }

    fn rotate_x(angle: f32) -> Transform {
        Mat4::from_rotation_x(angle)
    }

    fn rotate_y(angle: f32) -> Transform {
        Mat4::from_rotation_y(angle)
    }

    fn rotate_z(angle: f32) -> Transform {
        Mat4::from_rotation_z(angle)
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
