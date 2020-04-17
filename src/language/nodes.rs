pub use super::primitives::PrimitiveOperator;
pub use super::types::*;

use std::boxed::Box;
type OtherNode = Box<Node>;

#[derive(Clone)]
pub enum Node
{
    Nothing,
    Integer(IntegerNodeData),
    Boolean(BooleanNodeData),
    Variable(VariableNodeData),

    Call(CallNodeData),
    PrimitiveOperator(PrimitiveOperatorNodeData),

    Reference(ReferenceNodeData),
    Dereference(DereferenceNodeData),

    Binding(BindingNodeData),
    Assignment(AssignmentNodeData),

    Sequence(SequenceNodeData),
    Conditional(ConditionalNodeData),

    Function(FunctionNodeData),
}
impl Node
{
    pub fn from<T: ToNode>(value: T) -> Self
    {
        return value.to_node();
    }

    pub fn is_atomic(&self) -> bool
    {
        match self
        {
            Node::Nothing
            | Node::Integer(_)
            | Node::Boolean(_)
            | Node::Variable(_)
            | Node::PrimitiveOperator(_) => true,
            _ => false,
        }
    }

    pub fn is_definition(&self) -> bool
    {
        match self
        {
            Node::Function(_) => true,
            _ => false,
        }
    }
}
impl Typed for Node
{
    fn get_type(&self) -> &Type
    {
        match self
        {
            Node::Nothing => Type::void_ref(),
            Node::Integer(data) => data.get_type(),
            Node::Boolean(data) => data.get_type(),
            Node::Variable(data) => data.get_type(),

            Node::Call(data) => data.get_type(),
            Node::PrimitiveOperator(data) => data.get_type(),

            Node::Reference(data) => data.get_type(),
            Node::Dereference(data) => data.get_type(),

            Node::Binding(_) => Type::void_ref(),
            Node::Assignment(_) => Type::void_ref(),

            Node::Sequence(data) => data.get_type(),
            Node::Conditional(data) => data.get_type(),

            Node::Function(data) => data.get_type(),
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
    fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
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
            Node::Nothing
            | Node::Integer(_)
            | Node::Boolean(_)
            | Node::Variable(_)
            | Node::PrimitiveOperator(_) =>
            {}

            Node::Call(data) => data.recur_transformation(function, params),

            Node::Reference(data) => data.recur_transformation(function, params),
            Node::Dereference(data) => data.recur_transformation(function, params),

            Node::Binding(data) => data.recur_transformation(function, params),
            Node::Assignment(data) => data.recur_transformation(function, params),

            Node::Sequence(data) => data.recur_transformation(function, params),
            Node::Conditional(data) => data.recur_transformation(function, params),

            Node::Function(data) => data.recur_transformation(function, params),
        }
    }
    fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
    {
        match self
        {
            Node::Nothing
            | Node::Integer(_)
            | Node::Boolean(_)
            | Node::Variable(_)
            | Node::PrimitiveOperator(_) =>
            {}

            Node::Call(data) => data.recur_parse(function, params),

            Node::Reference(data) => data.recur_parse(function, params),
            Node::Dereference(data) => data.recur_parse(function, params),

            Node::Binding(data) => data.recur_parse(function, params),
            Node::Assignment(data) => data.recur_parse(function, params),

            Node::Sequence(data) => data.recur_parse(function, params),
            Node::Conditional(data) => data.recur_parse(function, params),

            Node::Function(data) => data.recur_parse(function, params),
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct BooleanNodeData
{
    value: bool,
}
impl BooleanNodeData
{
    pub fn get_value(&self) -> bool
    {
        return self.value;
    }

    pub fn new(value: bool) -> Self
    {
        return Self { value: value };
    }
}
impl Typed for BooleanNodeData
{
    fn get_type(&self) -> &Type
    {
        static BOOLEAN_TYPE: Type = Type::new_constant(DataType::Boolean);
        return &BOOLEAN_TYPE;
    }
}
impl_to_node!(BooleanNodeData, Node::Boolean);

#[derive(Clone)]
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
    pub fn set_name(&mut self, new_name: String)
    {
        self.name = new_name;
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

#[derive(Clone)]
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

    pub fn get_all_mut(&mut self) -> (&mut Node, &mut Vec<Node>, &mut Type)
    {
        return (
            self.operator.as_mut(),
            &mut self.operands,
            &mut self.node_type,
        );
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
    fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
    {
        function(self.get_operator(), params);
        for operand in self.get_operands().iter()
        {
            function(operand, params);
        }
    }
}
impl_typed!(CallNodeData);
impl_to_node!(CallNodeData, Node::Call);

#[derive(Clone)]
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
/*                               Reference Nodes                              */
/* -------------------------------------------------------------------------- */

#[derive(Clone)]
pub struct ReferenceNodeData
{
    reference_type: Reference,
    target_node:    OtherNode,

    node_type: Type,
}
impl ReferenceNodeData
{
    pub fn get_target(&self) -> &Node
    {
        return self.target_node.as_ref();
    }
    pub fn get_target_mut(&mut self) -> &mut Node
    {
        return self.target_node.as_mut();
    }

    pub fn get_reference_type(&self) -> Reference
    {
        return self.reference_type;
    }

    pub fn new(target_node: Node, reference_type: Reference) -> Self
    {
        return Self {
            target_node:    OtherNode::new(target_node),
            reference_type: reference_type,
            node_type:      Type::unknown(),
        };
    }
    pub fn new_infer_type(target_node: Node, reference_type: Reference) -> Self
    {
        return Self {
            node_type:      target_node.get_type().make_reference(reference_type),
            target_node:    OtherNode::new(target_node),
            reference_type: reference_type,
        };
    }
}
impl NodeRecur for ReferenceNodeData
{
    fn recur_transformation<TParams>(
        &mut self,
        function: fn(&mut Node, &mut TParams),
        params: &mut TParams,
    )
    {
        function(self.get_target_mut(), params);
    }
    fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
    {
        function(self.get_target(), params);
    }
}
impl_typed!(ReferenceNodeData);
impl_to_node!(ReferenceNodeData, Node::Reference);

#[derive(Clone)]
pub struct DereferenceNodeData
{
    target_node: OtherNode,
    node_type:   Type,
}
impl DereferenceNodeData
{
    pub fn get_target(&self) -> &Node
    {
        return self.target_node.as_ref();
    }
    pub fn get_target_mut(&mut self) -> &mut Node
    {
        return self.target_node.as_mut();
    }

    pub fn new(target_node: Node) -> Self
    {
        return Self {
            target_node: OtherNode::new(target_node),
            node_type:   Type::unknown(),
        };
    }
}
impl NodeRecur for DereferenceNodeData
{
    fn recur_transformation<TParams>(
        &mut self,
        function: fn(&mut Node, &mut TParams),
        params: &mut TParams,
    )
    {
        function(self.get_target_mut(), params);
    }
    fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
    {
        function(self.get_target(), params);
    }
}
impl_typed!(DereferenceNodeData);
impl_to_node!(DereferenceNodeData, Node::Dereference);

/* -------------------------------------------------------------------------- */
/*                              Structural Nodes                              */
/* -------------------------------------------------------------------------- */

#[derive(Clone)]
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
    fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
    {
        function(self.get_binding(), params);
    }
}
impl_to_node!(BindingNodeData, Node::Binding);

#[derive(Clone)]
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
    fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
    {
        function(self.get_lhs(), params);
        function(self.get_rhs(), params);
    }
}
impl_to_node!(AssignmentNodeData, Node::Assignment);

#[derive(Clone)]
pub struct SequenceNodeData
{
    nodes:          Vec<Node>,
    is_transparent: bool,
    node_type:      Type,
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

    pub fn get_final_node_index(&self) -> Option<usize>
    {
        if self.get_nodes().is_empty()
        {
            return None;
        }

        let mut final_index: Option<usize> = None;
        for (i, node) in self.get_nodes().iter().enumerate()
        {
            match node
            {
                Node::Function(_) =>
                {}
                _ =>
                {
                    final_index = Some(i);
                }
            }
        }

        if final_index == None
        {
            return Some(self.get_nodes().len() - 1);
        }
        else
        {
            return final_index;
        }
    }

    pub fn get_final_node(&self) -> Option<&Node>
    {
        return self.nodes.last();
    }
    pub fn get_final_node_mut(&mut self) -> Option<&mut Node>
    {
        return self.nodes.last_mut();
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
            node_type:      Type::unknown(),
        };
    }
    pub fn new_transparent(nodes: Vec<Node>) -> Self
    {
        return Self {
            nodes:          nodes,
            is_transparent: true,
            node_type:      Type::unknown(),
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
    fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
    {
        for node in self.get_nodes().iter()
        {
            function(node, params);
        }
    }
}
impl_typed!(SequenceNodeData);
impl_to_node!(SequenceNodeData, Node::Sequence);

#[derive(Clone)]
pub struct ConditionalNodeData
{
    condition_node: OtherNode,
    then_node:      OtherNode,
    else_node:      OtherNode,
}
impl ConditionalNodeData
{
    pub fn get_condition(&self) -> &Node
    {
        return self.condition_node.as_ref();
    }
    pub fn get_then(&self) -> &Node
    {
        return self.then_node.as_ref();
    }
    pub fn get_else(&self) -> &Node
    {
        return self.else_node.as_ref();
    }

    pub fn get_condition_mut(&mut self) -> &mut Node
    {
        return self.condition_node.as_mut();
    }
    pub fn get_then_mut(&mut self) -> &mut Node
    {
        return self.then_node.as_mut();
    }
    pub fn get_else_mut(&mut self) -> &mut Node
    {
        return self.else_node.as_mut();
    }

    pub fn has_else(&self) -> bool
    {
        if let Node::Nothing = self.get_else()
        {
            return false;
        }

        return true;
    }

    pub fn new(condition_node: Node, then_node: Node, else_node: Node) -> Self
    {
        return Self {
            condition_node: OtherNode::new(condition_node),
            then_node:      OtherNode::new(then_node),
            else_node:      OtherNode::new(else_node),
        };
    }
}
impl NodeRecur for ConditionalNodeData
{
    fn recur_transformation<TParams>(
        &mut self,
        function: fn(&mut Node, &mut TParams),
        params: &mut TParams,
    )
    {
        function(self.get_condition_mut(), params);
        function(self.get_then_mut(), params);
        function(self.get_else_mut(), params);
    }
    fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
    {
        function(self.get_condition(), params);
        function(self.get_then(), params);
        function(self.get_else(), params);
    }
}
impl Typed for ConditionalNodeData
{
    fn get_type(&self) -> &Type
    {
        return self.get_then().get_type();
    }
}
impl_to_node!(ConditionalNodeData, Node::Conditional);

/* -------------------------------------------------------------------------- */
/*                              Definition Nodes                              */
/* -------------------------------------------------------------------------- */

#[derive(Clone)]
pub struct ArgumentData
{
    name:          String,
    argument_type: Type,
}
impl ArgumentData
{
    pub fn get_name(&self) -> &String
    {
        return &self.name;
    }
    pub fn get_type(&self) -> &Type
    {
        return &self.argument_type;
    }

    pub fn new(name: String, argument_type: Type) -> Self
    {
        return Self {
            name:          name,
            argument_type: argument_type,
        };
    }
}

#[derive(Clone)]
pub struct FunctionNodeData
{
    name:      String,
    arguments: Vec<ArgumentData>,
    body_node: OtherNode,

    return_type: Type,
    node_type:   Type,
}
impl FunctionNodeData
{
    pub fn get_name(&self) -> &String
    {
        return &self.name;
    }
    pub fn set_name(&mut self, new_name: String)
    {
        self.name = new_name;
    }

    pub fn get_arguments(&self) -> &Vec<ArgumentData>
    {
        return &self.arguments;
    }
    pub fn get_return_type(&self) -> &Type
    {
        return &self.return_type;
    }

    pub fn get_body(&self) -> &Node
    {
        return self.body_node.as_ref();
    }
    pub fn get_body_mut(&mut self) -> &mut Node
    {
        return self.body_node.as_mut();
    }

    pub fn new(name: String, arguments: Vec<ArgumentData>, return_type: Type, body: Node) -> Self
    {
        let mut argument_types = Vec::new();
        for argument in arguments.iter()
        {
            argument_types.push(argument.get_type().clone());
        }
        let function_type = Type::from(CallableTypeData::new(argument_types, return_type.clone()));

        return Self {
            name:        name,
            arguments:   arguments,
            body_node:   OtherNode::new(body),
            node_type:   function_type,
            return_type: return_type,
        };
    }
    pub fn new_infer_type(name: String, arguments: Vec<ArgumentData>, body: Node) -> Self
    {
        return FunctionNodeData::new(name, arguments, body.get_type().clone(), body);
    }
}
impl NodeRecur for FunctionNodeData
{
    fn recur_transformation<TParams>(
        &mut self,
        function: fn(&mut Node, &mut TParams),
        params: &mut TParams,
    )
    {
        function(self.get_body_mut(), params);
    }
    fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
    {
        function(self.get_body(), params);
    }
}
impl_typed!(FunctionNodeData);
impl_to_node!(FunctionNodeData, Node::Function);
