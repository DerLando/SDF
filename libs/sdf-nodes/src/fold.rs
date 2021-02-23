use crate::{constant::Constant, node::{ArgsIter, ArgsIterMut, Node}, variable::{Variable, VariableType}};

pub(crate) trait Folder {
    fn fold_variable(&mut self) -> VariableType {VariableType::Variable}
    fn fold_constant(&mut self, c: Constant) -> VariableType {VariableType::Constant(c)}
    fn fold(&mut self, variable: VariableType) -> VariableType {
        match variable {
            VariableType::Variable => self.fold_variable(),
            VariableType::Constant(c) => self.fold_constant(c),
            VariableType::Node(n) => self.fold_node(n)
        }
    }
    fn fold_node(&mut self, node: Node) -> VariableType;
}

struct ConstantFolder;

// impl Folder for ConstantFolder {
//     fn fold_node(&mut self, node: Node) -> VariableType {
//         if node.args_iter().any(|v| v.is_variable()) {
//             VariableType::Node(node)
//         } else {
//             VariableType::Constant()
//         }

//     }
// }

fn children_mut(node: &mut Node) -> impl Iterator<Item = &mut Node> {
    node.args_iter_mut()
        .filter_map(|v| match v {
            VariableType::Node(n) => Some(n),
            _ => None
        })
}

fn children(node: &Node) -> impl Iterator<Item = &Node> {
    node.args_iter()
        .filter_map(|v| match v {
            VariableType::Node(n) => Some(n),
            _ => None
        })
}

fn all_children(node: &Node) -> impl Iterator<Item = &Node> {
    let mut all: Vec<&Node> = Vec::new();
    let direct: Vec<_> = children(node).collect();

    for child in direct.iter() {
        all.extend(all_children(child));
    }
    all.extend(direct);

    all.into_iter()
}

// walk down nodes, testing all children recursively if they contain variables
// if not, we can evaluate the current node and replace it with the result:
// *v = VariableType::Constant(v.operate(&Vec3::Default()))

#[cfg(test)]
mod test {
    use crate::SdfTree;

    use super::*;

    #[test]
    fn it_works() {
        let mut circle = SdfTree::circle(5.0);

        println!("{}", circle);

        assert!(false);
    }
}