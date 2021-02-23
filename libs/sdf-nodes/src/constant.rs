use std::rc::Rc;

use sdf_vecs::VecType;

pub(crate) type Constant = Rc<VecType>;

pub(crate) struct ConstantContainer {
    store: Vec<VecType>
}

impl Default for ConstantContainer {
    fn default() -> Self {
        ConstantContainer::new()
    }
}

impl ConstantContainer {
    pub const fn new() -> Self {
        Self {
            store: Vec::new()
        }
    }

    pub fn get_or_insert(&mut self, value: &VecType) -> Constant {
        if let Some(c) = self.store.iter().find(|c| *c == value) {
            Rc::new(*c)
        } else {
            self.store.push(*value);
            Rc::new(*self.store.last().unwrap())
        }
    }
}