mod add;
mod neg;
mod sub;
mod length;

enum OperatorKind {
    /// Scale all vecs to the highes dimension between them and operate afterwards
    High,
    /// Scale all vecs to the lowest dimension between them and operate afterwards
    Low
}

pub use add::{add_high, add_low};
pub use sub::{sub_high, sub_low};
pub use length::{Length};