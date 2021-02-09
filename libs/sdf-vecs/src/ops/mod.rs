mod add;
mod neg;
mod sub;
mod length;
mod max;
mod min;

enum OperatorKind {
    /// Scale all vecs to the highes dimension between them and operate afterwards
    High,
    /// Scale all vecs to the lowest dimension between them and operate afterwards
    Low
}

pub use add::{add_high, add_low};
pub use sub::{sub_high, sub_low};
pub use length::{Length};
pub use max::{max_high, max_low};
pub use min::{min_high, min_low};

use crate::{VecType, dimension::Dimension, scale::scale_n};

fn scale_same(a: &VecType, b: &VecType, kind: OperatorKind) -> (u8, VecType, VecType) {
    let l_dim = a.dimension();
    let r_dim = b.dimension();
    let dim = {
        match kind {
            OperatorKind::High => l_dim.max(r_dim),
            OperatorKind::Low => l_dim.min(r_dim)
        }
    };

    let lhs = scale_n(a, dim);
    let rhs = scale_n(b, dim);

    (dim, lhs, rhs)
}