use std::ops::DerefMut;

use sdf_vecs::{ComponentAccess, Vec1, Vec3, VecType};

use crate::{Spatial, ops::{Add, Length, Neg, NoOp}, primitives::{box_2d, box_3d, circle}};

pub struct TraitSDF {
    root: Box<dyn Spatial>
}

impl TraitSDF {
    pub(crate) fn new(sdf: Box<dyn Spatial>) -> Self {
        Self {
            root: sdf
        }
    }

    pub fn sign_at(&mut self, position: &Vec3) -> f32 {
        // insert position for variable
        self.root.deref_mut().replace_variable(position);

        // operate the whole tree and return
        match self.root.operate() {
            VecType::Vec1(v) => v.x(),
            _ => unreachable!()
        }

    }

    fn var_length() -> Self {
        let length = Length::new(Box::new(NoOp::new_var()));

        Self {
            root: Box::new(length)
        }
    }

    pub fn circle(center: &Vec3, radius: f32) -> Self {
        // length(P-C)-r, where P is query point, C is Center vec and r is radius
        circle(center, radius)
    }

    /// Create a rectangle centered at (0, 0), with max extents of x and y
    pub fn rectangle(width: f32, height: f32) -> Self {
        box_2d(width, height)
    }

    pub fn cuboid(x: f32, y: f32, z: f32) -> Self {
        box_3d(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_works() {
        let mut sdf = TraitSDF::var_length();

        assert_eq!(1.0, sdf.sign_at(&Vec3::new(1.0, 0.0, 0.0)));
        assert_eq!(1.0, sdf.sign_at(&Vec3::new(-1.0, 0.0, 0.0)));
        assert_eq!(2.0f32.sqrt(), sdf.sign_at(&Vec3::new(-1.0, 1.0, 0.0)));
    }

    #[test]
    fn circle_works() {
        let mut sdf = TraitSDF::circle(&Vec3::new(0.0, -1.0, 0.0), 10.0);

        assert_eq!(-10.0, sdf.sign_at(&Vec3::new(0.0, -1.0, 0.0)));
        assert_eq!(0.0, sdf.sign_at(&Vec3::new(10.0, -1.0, 0.0)));
        assert_eq!(10.0, sdf.sign_at(&Vec3::new(20.0, -1.0, 0.0)));
    }

    #[test]
    fn rectangle_works() {
        let mut rect = TraitSDF::rectangle(3.0, 6.0);

        assert_eq!(0.0, rect.sign_at(&Vec3::new(3.0, 6.0, 0.0)));
        assert_eq!(1.5, rect.sign_at(&Vec3::new(4.5, 3.0, 0.0)));
    }

    #[test]
    fn cuboid_works() {
        let mut cuboid = TraitSDF::cuboid(1.0, 2.0, 3.0);

        assert_eq!(0.0, cuboid.sign_at(&Vec3::new(1.0, 2.0, 3.0)));
        assert_eq!(2.0, cuboid.sign_at(&Vec3::new(3.0, 0.0, 0.0)));
    }
}
