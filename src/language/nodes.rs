pub use super::primitives::PrimitiveOperator;
pub use super::types::*;

use std::boxed::Box;
type OtherNode = Box<Node>;

pub enum Node
{
    Nothing,
    Integer(IntegerNodeData),
    Variable(VariableNodeData),

    Call(CallNodeData),
    PrimitiveOperator(PrimitiveOperatorNodeData),

    Binding(BindingNodeData),
    Assignment(AssignmentNodeData),
    Sequence(SequenceNodeData),
}
impl Node
{
    pub fn from<T: ToNode>(value: T) -> Self
    {
        return value.to_node();
    }
    pub fn parse_recursive<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
    {
        use Node::*;

        function(self, params);

        match self
        {
            Nothing | Integer(_) | Variable(_) =>
            {}

            Call(data) =>
            {
                data.get_operator().parse_recursive(function, params);
                for child in data.get_operands().iter()
                {
                    child.parse_recursive(function, params);
                }
            }
            PrimitiveOperator(_) =>
            {}

            Binding(data) =>
            {
                data.get_binding().parse_recursive(function, params);
            }
            Assignment(data) =>
            {
                data.get_lhs().parse_recursive(function, params);
                data.get_rhs().parse_recursive(function, params);
            }
            Sequence(data) =>
            {
                for child in data.get_nodes().iter()
                {
                    child.parse_recursive(function, params);
                }
            }
        }
    }
}
impl Typed for Node
{
    fn get_type(&self) -> &Type
    {
        match self
        {
            Node::Nothing => Type::unknown_ref(),
            Node::Integer(data) => data.get_type(),
            Node::Variable(data) => data.get_type(),

            Node::Call(data) => data.get_type(),
            Node::PrimitiveOperator(data) => data.get_type(),

            Node::Binding(_) => Type::unknown_ref(),
            Node::Assignment(_) => Type::unknown_ref(),
            Node::Sequence(data) => data.get_type(),
        }
    }
}
pub trait NodeRecur
{
    fn recur_transformation<TParams>(
        &mut self,
        function: fn(&mut Node, &mut TParams),
        params: &mut TParams,
    )
    {
        // Does nothing by default (no child nodes to recur into)
    }
}
impl NodeRecur for Node
{
    fn recur_transformation<TParams>(
        &mut self,
        function: fn(&mut Node, &mut TParams),
        params: &mut TParams,
    )
    {
        match self
        {
            Node::Nothing =>
            {}
            Node::Integer(data) =>
            {}
            Node::Variable(data) =>
            {}
            Node::Call(data) => data.recur_transformation(function, params),
            Node::PrimitiveOperator(data) =>
            {}
            Node::Binding(data) => data.recur_transformation(function, params),
            Node::Assignment(data) => data.recur_transformation(function, params),
            Node::Sequence(data) => data.recur_transformation(function, params),
        }
    }
}

pub trait ToNode
{
    fn to_node(self) -> Node;
}

/* -------------------------------------------------------------------------- */
/*                                Helper Macros                               */
/* -------------------------------------------------------------------------- */
macro_rules! impl_typed {
    ($target:path) => {
        impl Typed for $target
        {
            fn get_type(&self) -> &Type
            {
                return &self.node_type;
            }
        }
        impl TypedInferred for $target
        {
            fn set_type(&mut self, new_type: Type)
            {
                self.node_type = new_type;
            }
        }
    };
}

macro_rules! impl_to_node {
    ($target:path, $node_type:path) => {
        impl ToNode for $target
        {
            fn to_node(self) -> Node
            {
                return $node_type(self);
            }
        }
    };
}

/* -------------------------------------------------------------------------- */
/*                                Atomic Nodes                                */
/* -------------------------------------------------------------------------- */

pub struct IntegerNodeData
{
    value: i64,
}
impl IntegerNodeData
{
    pub fn get_value(&self) -> i64
    {
        return self.value;
    }

    pub fn new(value: i64) -> Self
    {
        return Self { value: value };
    }
}
impl Typed for IntegerNodeData
{
    fn get_type(&self) -> &Type
    {
        static INTEGER_TYPE: Type = Type::new_constant(DataType::Integer);
        return &INTEGER_TYPE;
    }
}
impl_to_node!(IntegerNodeData, Node::Integer);

pub struct VariableNodeData
{
    name:      String,
    node_type: Type,
}
impl VariableNodeData
{
    pub fn get_name(&self) -> &String
    {
        return &self.name;
    }

    pub fn new(name: String) -> Self
    {
        return Self {
            name:      name,
            node_type: Type::unknown(),
        };
    }
    pub fn new_typed(name: String, node_type: Type) -> Self
    {
        return Self {
            name:      name,
            node_type: node_type,
        };
    }
}
impl_typed!(VariableNodeData);
impl_to_node!(VariableNodeData, Node::Variable);

/* -------------------------------------------------------------------------- */
/*                                  Functions                                 */
/* -------------------------------------------------------------------------- */

pub struct CallNodeData
{
    operator: OtherNode,
    operands: Vec<Node>,

    node_type: Type,
}
impl CallNodeData
{
    pub fn get_operator(&self) -> &Node
    {
        return self.operator.as_ref();
    }
    pub fn get_operands(&self) -> &Vec<Node>
    {
        return &self.operands;
    }
    pub fn get_operator_mut(&mut self) -> &mut Node
    {
        return self.operator.as_mut();
    }
    pub fn get_operands_mut(&mut self) -> &mut Vec<Node>
    {
        return &mut self.operands;
    }

    pub fn new(operator: Node, operands: Vec<Node>) -> Self
    {
        return Self {
            node_type: Type::unknown(),
            operator:  OtherNode::new(operator),
            operands:  operands,
        };
    }
}
impl NodeRecur for CallNodeData
{
    fn recur_transformation<TParams>(
        &mut self,
        function: fn(&mut Node, &mut TParams),
        params: &mut TParams,
    )
    {
        function(self.get_operator_mut(), params);
        for operand in self.get_operands_mut().iter_mut()
        {
            function(operand, params);
        }
    }
}
impl_typed!(CallNodeData);
impl_to_node!(CallNodeData, Node::Call);

pub struct PrimitiveOperatorNodeData
{
    node_type: Type,
    operator:  PrimitiveOperator,
}
impl PrimitiveOperatorNodeData
{
    pub fn get_operator(&self) -> PrimitiveOperator
    {
        return self.operator;
    }

    pub fn new(operator: PrimitiveOperator) -> Self
    {
        return Self {
            node_type: Type::unknown(),
            operator:  operator,
        };
    }
}
impl_typed!(PrimitiveOperatorNodeData);
impl_to_node!(PrimitiveOperatorNodeData, Node::PrimitiveOperator);

/* -------------------------------------------------------------------------- */
/*                              Structural Nodes                              */
/* -------------------------------------------------------------------------- */

pub struct BindingNodeData
{
    name:         String,
    binding_node: OtherNode,

    binding_type: Type,
}
impl BindingNodeData
{
    pub fn get_name(&self) -> &String
    {
        return &self.name;
    }
    pub fn get_binding(&self) -> &Node
    {
        return self.binding_node.as_ref();
    }
    pub fn get_binding_mut(&mut self) -> &mut Node
    {
        return self.binding_node.as_mut();
    }

    pub fn get_binding_type(&self) -> &Type
    {
        return &self.binding_type;
    }
    pub fn set_binding_type(&mut self, new_type: Type)
    {
        self.binding_type = new_type;
    }

    pub fn new(name: String, binding_node: Node) -> Self
    {
        return Self {
            name:         name,
            binding_type: binding_node.get_type().clone(),
            binding_node: OtherNode::new(binding_node),
        };
    }
    pub fn new_empty(name: String, binding_type: Type) -> Self
    {
        return Self {
            name:         name,
            binding_node: OtherNode::new(Node::Nothing),
            binding_type: binding_type,
        };
    }
}
impl NodeRecur for BindingNodeData
{
    fn recur_transformation<TParams>(
        &mut self,
        function: fn(&mut Node, &mut TParams),
        params: &mut TParams,
    )
    {
        function(self.get_binding_mut(), params);
    }
}
impl_to_node!(BindingNodeData, Node::Binding);

pub struct AssignmentNodeData
{
    lhs: OtherNode,
    rhs: OtherNode,
}
impl AssignmentNodeData
{
    pub fn get_rhs(&self) -> &Node
    {
        return self.rhs.as_ref();
    }
    pub fn get_rhs_mut(&mut self) -> &mut Node
    {
        return self.rhs.as_mut();
    }
    pub fn get_lhs(&self) -> &Node
    {
        return self.lhs.as_ref();
    }
    pub fn get_lhs_mut(&mut self) -> &mut Node
    {
        return self.lhs.as_mut();
    }

    pub fn new(lhs: Node, rhs: Node) -> Self
    {
        return Self {
            lhs: OtherNode::new(lhs),
            rhs: OtherNode::new(rhs),
        };
    }
}
impl NodeRecur for AssignmentNodeData
{
    fn recur_transformation<TParams>(
        &mut self,
        function: fn(&mut Node, &mut TParams),
        params: &mut TParams,
    )
    {
        function(self.get_lhs_mut(), params);
        function(self.get_rhs_mut(), params);
    }
}
impl_to_node!(AssignmentNodeData, Node::Assignment);

pub struct SequenceNodeData
{
    nodes:          Vec<Node>,
    is_transparent: bool,
}
impl SequenceNodeData
{
    pub fn get_nodes(&self) -> &Vec<Node>
    {
        return &self.nodes;
    }
    pub fn get_nodes_mut(&mut self) -> &mut Vec<Node>
    {
        return &mut self.nodes;
    }
    pub fn is_transparent(&self) -> bool
    {
        return self.is_transparent;
    }

    pub fn new(nodes: Vec<Node>) -> Self
    {
        return Self {
            nodes:          nodes,
            is_transparent: false,
        };
    }
    pub fn new_transparent(nodes: Vec<Node>) -> Self
    {
        return Self {
            nodes:          nodes,
            is_transparent: true,
        };
    }
}
impl NodeRecur for SequenceNodeData
{
    fn recur_transformation<TParams>(
        &mut self,
        function: fn(&mut Node, &mut TParams),
        params: &mut TParams,
    )
    {
        for node in self.get_nodes_mut().iter_mut()
        {
            function(node, params);
        }
    }
}
impl Typed for SequenceNodeData
{
    fn get_type(&self) -> &Type
    {
        if let Some(node) = self.nodes.last()
        {
            return node.get_type();
        }
        else
        {
            static VOID_TYPE: Type = Type::new_constant(DataType::Void);
            return &VOID_TYPE;
        }
    }
}
impl_to_node!(SequenceNodeData, Node::Sequence);
