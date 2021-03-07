# Sdf-nodes

Node-based, *tree-like* representation of `SDF`

# TODO

 - [x] remove `Constant` store, the gain should be minimal and it's showing pain points
 - [x] impl Clone for ops
 - [ ] implement public facing ops of `SdfTree` as internal operations, f.e. *union*, or *scale*
 - [x] implement tree simplification

# Notes

The node design can't be simplified, the Node is already the tree as we want it, `SdfTree` is only the public interface. This means we should change all it's methods to just wrappers around operations.

Still need to implement a nice and clean way to simplify a tree, as I feel immutable trees allow for a cleaner and faster design, the simplify method should just create a new, simplified tree.
The simplification, should not be complete yet, I think a tree won't necesserally be simplified recursively to it's smallest form.