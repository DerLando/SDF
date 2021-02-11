use sdf_vecs::{Vec3, VecType};

use crate::{Operator, Spatial};

/// The way to introduce variables into the SDF tree
#[derive(Clone)]
pub(crate) struct Variable;

impl Operator for Variable {
    fn operate(&self, pos: &Vec3) -> VecType {
        VecType::Vec3(*pos)
    }
}

impl Spatial for Variable { }