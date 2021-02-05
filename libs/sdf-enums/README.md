# Enum based SDF

This will be the *mvp* for enum-based implementation of *Signed distance fields*

# Design

We compose operations as enums that store their values to operate on.

## Operators

Basically all operators will be stored in a big enum. Their arguments will be stored on the enum variants itself indirectly.

For this to work *flexibly*, we need *variable* enums, too:

 - `VecVar`, either a `Vec` or a `Replacable(`Vec3`)`,
 - `NumConst`, an f32

For now, operators are ordered by their return type, which gives 3 enums instead of one. I can't think of a good way to construct them from a big enum.

When implementing the `Circle` primitive, this approach really falls apart. `Length` only takes `Vec` as input, and we don't have any way to down-cast from a `VariableReturningOperator` without destroying the variable in the process.
The only way around this is to implement f.e. `Add` 4 times:

 - `Add(Vec, Vec)`, a `VecReturning` variant
 - `AddNum(Vec, Number)`, a `VecReturning` variant
 - `NumAdd(Number, Vec)`, a `VecReturning` variant
 - `Add(Number, Number)` a `NumberReturning` variant

This seems **really** contrived and gives me doubts if this is even the right approach to take.