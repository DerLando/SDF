use std::ops::{Deref, DerefMut};

use sdf_vecs::{ComponentAccess, Vec1, Vec3, VecType};

use crate::{Spatial, ops::{Constant, Variable, add, length, max, min, mul, sub}, primitives::{box_2d, box_3d, circle, torus}};

#[derive(Clone)]
pub struct TraitSDF;

impl TraitSDF {
    pub fn sign_at(sdf: &impl Spatial, position: &Vec3) -> f32 {
        // operate the whole tree and return
        match sdf.operate(position) {
            VecType::Vec1(v) => v.x(),
            _ => unreachable!()
        }

    }

    pub fn circle(center: &Vec3, radius: f32) -> impl Spatial {
        // length(P-C)-r, where P is query point, C is Center vec and r is radius
        circle(center, radius)
    }

    pub fn sphere(radius: f32) -> impl Spatial {
        let r: Constant = radius.into();
        sub(length(Variable), r)
    }

    /// Create a rectangle centered at (0, 0), with max extents of x and y
    pub fn rectangle(width: f32, height: f32) -> impl Spatial {
        box_2d(width, height)
    }

    pub fn cuboid(x: f32, y: f32, z: f32) -> impl Spatial {
        box_3d(x, y, z)
    }

    pub fn union(a: impl Spatial + 'static, b: impl Spatial + 'static) -> impl Spatial {
        min(a, b)
    }

    pub fn intersection(a: impl Spatial + 'static, b: impl Spatial + 'static) -> impl Spatial {
        max(a, b)
    }

    pub fn blend(a: impl Spatial + 'static, b: impl Spatial + 'static, factor: f32) -> impl Spatial {
        // a is blend factor, L is left tree and R is right tree
        // d = (1 - a) * L + a * R

        let factor: Constant = factor.into();
        let one: Constant = 1.0.into();

        let lhs = mul(sub(one, factor.clone()), a);
        let rhs = mul(factor, b);

        add(lhs, rhs)
    }

    pub fn rounded_edges(sdf: impl Spatial + 'static, radius: f32) -> impl Spatial {
        let r: Constant = radius.into();
        sub(sdf, r)
    }

    pub fn torus(inner_radius: f32, outer_radius: f32) -> impl Spatial {
        torus(inner_radius, outer_radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_works() {
        let mut sdf = TraitSDF::circle(&Vec3::new(0.0, -1.0, 0.0), 10.0);

        assert_eq!(-10.0, TraitSDF::sign_at(&mut sdf, &Vec3::new(0.0, -1.0, 0.0)));
        assert_eq!(0.0, TraitSDF::sign_at(&mut sdf, &Vec3::new(10.0, -1.0, 0.0)));
        assert_eq!(10.0, TraitSDF::sign_at(&mut sdf, &Vec3::new(20.0, -1.0, 0.0)));
    }

    // #[test]
    // fn rectangle_works() {
    //     let mut rect = TraitSDF::rectangle(3.0, 6.0);

    //     assert_eq!(0.0, rect.sign_at(&Vec3::new(3.0, 6.0, 0.0)));
    //     assert_eq!(1.5, rect.sign_at(&Vec3::new(4.5, 3.0, 0.0)));
    // }

    // #[test]
    // fn cuboid_works() {
    //     let mut cuboid = TraitSDF::cuboid(1.0, 2.0, 3.0);

    //     assert_eq!(0.0, cuboid.sign_at(&Vec3::new(1.0, 2.0, 3.0)));
    //     assert_eq!(2.0, cuboid.sign_at(&Vec3::new(3.0, 0.0, 0.0)));
    // }
}
