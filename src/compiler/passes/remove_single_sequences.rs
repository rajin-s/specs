use crate::language::nodes::*;
use std::mem::swap;

pub fn apply(root_node: &mut Node)
{
    let mut params = ();
    root_node.recur_transformation(remove_singles, &mut params);
}

fn remove_singles(node: &mut Node, _params: &mut ())
{
    match node
    {
        Node::Sequence(data) =>
        {
            if data.get_nodes().len() == 1
            {
                let mut temp = Node::Nothing;
                swap(&mut temp, node);

                if let Node::Sequence(mut sequence_data) = temp
                {
                    swap(&mut sequence_data.get_nodes_mut()[0], node);
                }
                else
                {
                    unreachable!();
                }
            }
        }
        _ =>
        {}
    }

    node.recur_transformation(remove_singles, _params);
}
