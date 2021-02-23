use glam::{Mat4, Vec3};

pub type Transform = Mat4;

pub struct TransformHelper;

pub enum RotationAxis {
    X,
    Y,
    Z
}


/// Create transforms
impl TransformHelper {
    pub fn identity() -> Transform {
        Mat4::default()
    }

    pub fn translation(v: &Vec3) -> Transform {
        Mat4::from_translation(*v)
    }

    pub fn rotate(angle: f32, axis: RotationAxis) -> Transform {
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