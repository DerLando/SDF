mod add;
mod neg;
mod sub;

enum OperatorKind {
    /// Scale all vecs to the highes dimension between them and operate afterwards
    High,
    /// Scale all vecs to the lowest dimension between them and operate afterwards
    Low
}