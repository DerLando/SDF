# Trait-based SDF

This is the *mvp* for a trait-based implementation of *Signed distance fields*.

# Design

We compose *operations* on *values* as trait objects.

## Operators

The main traits are:
 - `Operator<T>`, which gives the `operate(&self) -> T` method. All other operator traits derive from this
 - `Algebraic`, which implements `Operator<f32>`. All operators that always return a number, f.e. *Length*, or *Dot*
 - `Spatial`, which implements `Operator<Vec>`. All operators that always return a `Vec`, f.e. *YX*, *Cross*, or *Vec4*
 - `Mixed`, which implements `Operator<EvaluatedVariable>`. All operators where the return type can be either a `Vec` or a number, f.e. *Add*, or *NoOp*

This is **very** flexible and allows us to precisely model the very different arguments needed for different operations.
*Clamp*, *Swizzling*, *Atan2* and *Length* f.e. have vastly differing input requirements and we need to be able to abstract over all of them.

## Sign

To be able to dynamically evaluate the sign of any given position in space, operators can store a `Replacable(Vec3)` enum Variant and implement the `VariableContainer` trait.

The variants to describe dynamic variables inside operators are:
 - `Box<dyn Algebraic>`, if your argument will only ever evaluate to a number, f.e. *Atan2*
 - `VecVar`, if your argument will only ever evaluate to a `Vec`. Can also be a `Replaceable(Vec3), f.e. *Cross*
 - `Variable`, if your argument can be either evaluate to a number, or be a VecVar, f.e. *Add*

## SDF interface

Users can interface via the public `SDF` struct, which exposes the `SDF::sign_at(&mut self, position: &Vec3)` method. It is implemented like so:

```rust
struct SDF {
    // by forcing root to be an `Algebraic` trait object, we know for certain that it will evaluate to an f32
    root: Box<dyn Algebraic>
}
```

When calling the `SDF::sign_at(&mut self, position: &Vec3)` function, the SDF will first traverse down it's operation tree, replacing all occurences of `Replacable(Vec3)` with the given position. Afterwards it will call it's roots `operate` method, which will recursively evaluate all operations and ultimately return the correct sign.

While mutable access to the `SDF` is quite unflexible, it could only be circumvented by cloning the whole `SDF` on every call to `sign_at` and manipulating the copy. As an `SDF` is typically evaluated for pixels or voxels, this would mean **millions** of additional allocations.

## Cons

Since we are passing around trait objects, everything is *boxed* and stored on the heap. This probably won't be too nice for performance overall, we will need to benchmark this against other implementations.

# TODO

- [ ] Make `VariableContainer` optional instead of forcing implementation on all *operators*. *Atan2*, or *Pow* can never store a variable and we should be able to represent this through composition.