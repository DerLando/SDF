# Sdf-nodes

Node-based, *tree-like* representation of `SDF`

# TODO

 - [ ] remove `Constant` store, the gain should be minimal and it's showing pain points
 - [ ] simplify the node design, a tree should combine a node with transformation meta-data, nothing else!
 - [ ] make trees mutable *in-place*
 - [ ] let all ops operate on trees, instead of nodes
 - [ ] impl Clone for ops