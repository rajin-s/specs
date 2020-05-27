// pub use super::primitive;
// pub use super::types::*;

// use std::boxed::Box;
// type OtherNode = Box<Node>;

// #[derive(Clone)]
// pub enum Node
// {
//     Nothing,
//     Integer(IntegerNodeData),
//     Boolean(BooleanNodeData),
//     Variable(VariableNodeData),

//     Call(CallNodeData),
//     PrimitiveOperator(PrimitiveOperatorNodeData),

//     Reference(ReferenceNodeData),
//     Dereference(DereferenceNodeData),

//     Binding(BindingNodeData),
//     Assignment(AssignmentNodeData),
//     Sequence(SequenceNodeData),
//     Conditional(ConditionalNodeData),
//     Function(FunctionNodeData),

//     Type(TypeNodeData),
//     Access(AccessNodeData),
// }
// impl Node
// {
//     pub fn from<T: ToNode>(value: T) -> Self
//     {
//         return value.to_node();
//     }

//     pub fn is_atomic(&self) -> bool
//     {
//         match self
//         {
//             Node::Nothing
//             | Node::Integer(_)
//             | Node::Boolean(_)
//             | Node::Variable(_)
//             | Node::PrimitiveOperator(_) => true,
//             _ => false,
//         }
//     }

//     pub fn is_definition(&self) -> bool
//     {
//         match self
//         {
//             Node::Function(_) => true,
//             _ => false,
//         }
//     }
// }

// impl Typed for Node
// {
//     fn get_type(&self) -> &Type
//     {
//         match self
//         {
//             Node::Nothing | Node::Binding(_) | Node::Assignment(_) => basic_types::void(),

//             Node::Integer(data) => data.get_type(),
//             Node::Boolean(data) => data.get_type(),
//             Node::Variable(data) => data.get_type(),

//             Node::Call(data) => data.get_type(),
//             Node::PrimitiveOperator(data) => data.get_type(),

//             Node::Reference(data) => data.get_type(),
//             Node::Dereference(data) => data.get_type(),

//             Node::Sequence(data) => data.get_type(),
//             Node::Conditional(data) => data.get_type(),

//             Node::Function(data) => data.get_type(),

//             Node::Type(data) => data.get_type(),
//             Node::Access(data) => data.get_type(),
//         }
//     }
// }
// pub trait NodeRecur
// {
//     fn recur_transformation<TParams>(
//         &mut self,
//         _function: fn(&mut Node, &mut TParams),
//         _params: &mut TParams,
//     )
//     {
//         // Does nothing by default (no child nodes to recur into)
//     }
//     fn recur_parse<TParams>(&self, _function: fn(&Node, &mut TParams), _params: &mut TParams)
//     {
//         // Does nothing by default (no child nodes to recur into)
//     }
// }
// pub trait ToNode
// {
//     fn to_node(self) -> Node;
// }

// macro_rules! generate_node_recur {
//     (terminal : [$( $terminal:path ),*], data : [$( $data:path),*]) => {
//         fn recur_transformation<TParams>(&mut self, function: fn(&mut Node, &mut TParams), params: &mut TParams)
//         {
//             match self
//             {
//                 Node::Nothing => {},
//                 $( $terminal(_) => {}, )*
//                 $( $data(data) => data.recur_transformation(function,params), )*
//             }
//         }
//         fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
//         {
//             match self
//             {
//                 Node::Nothing => {},
//                 $( $terminal(_) => {}, )*
//                 $( $data(data) => data.recur_parse(function,params), )*
//             }
//         }
//     }
// }
// impl NodeRecur for Node
// {
//     generate_node_recur!(
//         terminal : [
//             Node::Integer,
//             Node::Boolean,
//             Node::Variable,
//             Node::PrimitiveOperator
//         ],
//         data : [
//             Node::Call,
//             Node::Conditional,
//             Node::Reference,
//             Node::Dereference,
//             Node::Binding,
//             Node::Assignment,
//             Node::Sequence,
//             Node::Function,
//             Node::Type,
//             Node::Access
//         ]
//     );
// }

// /* -------------------------------------------------------------------------- */
// /*                                Helper Macros                               */
// /* -------------------------------------------------------------------------- */
// macro_rules! impl_typed {
//     ($target:path) => {
//         impl Typed for $target
//         {
//             fn get_type(&self) -> &Type
//             {
//                 return &self.node_type;
//             }
//         }
//         impl TypedInferred for $target
//         {
//             fn set_type(&mut self, new_type: Type)
//             {
//                 self.node_type = new_type;
//             }
//         }
//     };
// }

// macro_rules! impl_to_node {
//     ($target:path, $node_type:path) => {
//         impl ToNode for $target
//         {
//             fn to_node(self) -> Node
//             {
//                 return $node_type(self);
//             }
//         }
//     };
// }

// /* -------------------------------------------------------------------------- */
// /*                                Atomic Nodes                                */
// /* -------------------------------------------------------------------------- */

// #[derive(Clone)]
// pub struct IntegerNodeData
// {
//     value: i64,
// }
// impl IntegerNodeData
// {
//     get!(value : get_value -> i64);

//     pub fn new(value: i64) -> Self
//     {
//         return Self { value: value };
//     }
// }
// impl Typed for IntegerNodeData
// {
//     fn get_type(&self) -> &Type
//     {
//         return basic_types::integer();
//     }
// }
// impl_to_node!(IntegerNodeData, Node::Integer);

// #[derive(Clone)]
// pub struct BooleanNodeData
// {
//     value: bool,
// }
// impl BooleanNodeData
// {
//     get!(value : get_value -> bool);

//     pub fn new(value: bool) -> Self
//     {
//         return Self { value: value };
//     }
// }
// impl Typed for BooleanNodeData
// {
//     fn get_type(&self) -> &Type
//     {
//         return basic_types::boolean();
//     }
// }
// impl_to_node!(BooleanNodeData, Node::Boolean);

// #[derive(Clone)]
// pub struct VariableNodeData
// {
//     name:      String,
//     node_type: Type,
// }
// impl VariableNodeData
// {
//     get!(name : get_name -> &String);

//     pub fn set_name(&mut self, new_name: String)
//     {
//         self.name = new_name;
//     }

//     pub fn new(name: String) -> Self
//     {
//         return Self {
//             name:      name,
//             node_type: Type::unknown(),
//         };
//     }
//     pub fn new_typed(name: String, node_type: Type) -> Self
//     {
//         return Self {
//             name:      name,
//             node_type: node_type,
//         };
//     }
// }
// impl_typed!(VariableNodeData);
// impl_to_node!(VariableNodeData, Node::Variable);

// /* -------------------------------------------------------------------------- */
// /*                                  Functions                                 */
// /* -------------------------------------------------------------------------- */

// #[derive(Clone)]
// pub struct CallNodeData
// {
//     operator: OtherNode,
//     operands: Vec<Node>,

//     node_type: Type,
// }
// impl CallNodeData
// {
//     get!(operator : get_operator -> &Node);
//     get!(operands : get_operands -> &Vec<Node>);
//     get!(operator : get_operator_mut -> &mut Node);
//     get!(operands : get_operands_mut -> &mut Vec<Node>);

//     pub fn get_all_mut(&mut self) -> (&mut Node, &mut Vec<Node>, &mut Type)
//     {
//         return (
//             self.operator.as_mut(),
//             &mut self.operands,
//             &mut self.node_type,
//         );
//     }

//     pub fn new(operator: Node, operands: Vec<Node>) -> Self
//     {
//         return Self {
//             node_type: Type::unknown(),
//             operator:  OtherNode::new(operator),
//             operands:  operands,
//         };
//     }
// }
// impl NodeRecur for CallNodeData
// {
//     fn recur_transformation<TParams>(
//         &mut self,
//         function: fn(&mut Node, &mut TParams),
//         params: &mut TParams,
//     )
//     {
//         function(self.get_operator_mut(), params);
//         for operand in self.get_operands_mut().iter_mut()
//         {
//             function(operand, params);
//         }
//     }
//     fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
//     {
//         function(self.get_operator(), params);
//         for operand in self.get_operands().iter()
//         {
//             function(operand, params);
//         }
//     }
// }
// impl_typed!(CallNodeData);
// impl_to_node!(CallNodeData, Node::Call);

// #[derive(Clone)]
// pub struct PrimitiveOperatorNodeData
// {
//     node_type: Type,
//     operator:  primitive::Operator,
// }
// impl PrimitiveOperatorNodeData
// {
//     get!(operator : get_operator -> primitive::Operator);

//     pub fn new(operator: primitive::Operator) -> Self
//     {
//         return Self {
//             node_type: Type::unknown(),
//             operator:  operator,
//         };
//     }
// }
// impl_typed!(PrimitiveOperatorNodeData);
// impl_to_node!(PrimitiveOperatorNodeData, Node::PrimitiveOperator);

// /* -------------------------------------------------------------------------- */
// /*                               Reference Nodes                              */
// /* -------------------------------------------------------------------------- */

// #[derive(Clone)]
// pub struct ReferenceNodeData
// {
//     reference_type: ReferenceMode,
//     target_node:    OtherNode,

//     node_type: Type,
// }
// impl ReferenceNodeData
// {
//     get!(target_node : get_target -> &Node);
//     get!(target_node : get_target_mut -> &mut Node);

//     get!(reference_type : get_reference_type -> ReferenceMode);

//     pub fn new(target_node: Node, reference_type: ReferenceMode) -> Self
//     {
//         return Self {
//             target_node:    OtherNode::new(target_node),
//             reference_type: reference_type,
//             node_type:      Type::unknown(),
//         };
//     }
//     pub fn new_infer_type(target_node: Node, reference_type: ReferenceMode) -> Self
//     {
//         return Self {
//             node_type:      target_node.get_type().make_reference(reference_type),
//             target_node:    OtherNode::new(target_node),
//             reference_type: reference_type,
//         };
//     }
// }
// impl NodeRecur for ReferenceNodeData
// {
//     fn recur_transformation<TParams>(
//         &mut self,
//         function: fn(&mut Node, &mut TParams),
//         params: &mut TParams,
//     )
//     {
//         function(self.get_target_mut(), params);
//     }
//     fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
//     {
//         function(self.get_target(), params);
//     }
// }
// impl_typed!(ReferenceNodeData);
// impl_to_node!(ReferenceNodeData, Node::Reference);

// #[derive(Clone)]
// pub struct DereferenceNodeData
// {
//     target_node: OtherNode,
//     node_type:   Type,
// }
// impl DereferenceNodeData
// {
//     get!(target_node : get_target -> &Node);
//     get!(target_node : get_target_mut -> &mut Node);

//     pub fn new(target_node: Node) -> Self
//     {
//         return Self {
//             target_node: OtherNode::new(target_node),
//             node_type:   Type::unknown(),
//         };
//     }
// }
// impl NodeRecur for DereferenceNodeData
// {
//     fn recur_transformation<TParams>(
//         &mut self,
//         function: fn(&mut Node, &mut TParams),
//         params: &mut TParams,
//     )
//     {
//         function(self.get_target_mut(), params);
//     }
//     fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
//     {
//         function(self.get_target(), params);
//     }
// }
// impl_typed!(DereferenceNodeData);
// impl_to_node!(DereferenceNodeData, Node::Dereference);

// /* -------------------------------------------------------------------------- */
// /*                              Structural Nodes                              */
// /* -------------------------------------------------------------------------- */

// #[derive(Clone)]
// pub struct BindingNodeData
// {
//     name:         String,
//     binding_node: OtherNode,

//     binding_type: Type,
// }
// impl BindingNodeData
// {
//     get!(name : get_name -> &String);

//     get!(binding_node : get_binding -> &Node);
//     get!(binding_node : get_binding_mut -> &mut Node);

//     get!(binding_type : get_binding_type -> &Type);
//     pub fn set_binding_type(&mut self, new_type: Type)
//     {
//         self.binding_type = new_type;
//     }

//     pub fn new(name: String, binding_node: Node) -> Self
//     {
//         return Self {
//             name:         name,
//             binding_type: binding_node.get_type().clone(),
//             binding_node: OtherNode::new(binding_node),
//         };
//     }
//     pub fn new_empty(name: String, binding_type: Type) -> Self
//     {
//         return Self {
//             name:         name,
//             binding_node: OtherNode::new(Node::Nothing),
//             binding_type: binding_type,
//         };
//     }
// }
// impl NodeRecur for BindingNodeData
// {
//     fn recur_transformation<TParams>(
//         &mut self,
//         function: fn(&mut Node, &mut TParams),
//         params: &mut TParams,
//     )
//     {
//         function(self.get_binding_mut(), params);
//     }
//     fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
//     {
//         function(self.get_binding(), params);
//     }
// }
// impl_to_node!(BindingNodeData, Node::Binding);

// #[derive(Clone)]
// pub struct AssignmentNodeData
// {
//     lhs: OtherNode,
//     rhs: OtherNode,
// }
// impl AssignmentNodeData
// {
//     get!(lhs : get_lhs -> &Node);
//     get!(lhs : get_lhs_mut -> &mut Node);
//     get!(rhs : get_rhs -> &Node);
//     get!(rhs : get_rhs_mut -> &mut Node);

//     pub fn new(lhs: Node, rhs: Node) -> Self
//     {
//         return Self {
//             lhs: OtherNode::new(lhs),
//             rhs: OtherNode::new(rhs),
//         };
//     }
// }
// impl NodeRecur for AssignmentNodeData
// {
//     fn recur_transformation<TParams>(
//         &mut self,
//         function: fn(&mut Node, &mut TParams),
//         params: &mut TParams,
//     )
//     {
//         function(self.get_lhs_mut(), params);
//         function(self.get_rhs_mut(), params);
//     }
//     fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
//     {
//         function(self.get_lhs(), params);
//         function(self.get_rhs(), params);
//     }
// }
// impl_to_node!(AssignmentNodeData, Node::Assignment);

// #[derive(Clone)]
// pub struct SequenceNodeData
// {
//     nodes:          Vec<Node>,
//     is_transparent: bool,
//     node_type:      Type,
// }
// impl SequenceNodeData
// {
//     get!(nodes : get_nodes -> &Vec<Node>);
//     get!(nodes : get_nodes_mut -> &mut Vec<Node>);

//     pub fn get_final_node_index(&self) -> Option<usize>
//     {
//         if self.get_nodes().is_empty()
//         {
//             return None;
//         }

//         let mut final_index: Option<usize> = None;
//         for (i, node) in self.get_nodes().iter().enumerate()
//         {
//             match node
//             {
//                 Node::Function(_) =>
//                 {}
//                 _ =>
//                 {
//                     final_index = Some(i);
//                 }
//             }
//         }

//         if final_index == None
//         {
//             return Some(self.get_nodes().len() - 1);
//         }
//         else
//         {
//             return final_index;
//         }
//     }

//     pub fn get_final_node(&self) -> Option<&Node>
//     {
//         match self.get_final_node_index()
//         {
//             Some(index) => self.nodes.get(index),
//             None => None,
//         }
//     }
//     pub fn get_final_node_mut(&mut self) -> Option<&mut Node>
//     {
//         match self.get_final_node_index()
//         {
//             Some(index) => self.nodes.get_mut(index),
//             None => None,
//         }
//     }

//     pub fn is_transparent(&self) -> bool
//     {
//         return self.is_transparent;
//     }

//     pub fn new(nodes: Vec<Node>) -> Self
//     {
//         return Self {
//             nodes:          nodes,
//             is_transparent: false,
//             node_type:      Type::unknown(),
//         };
//     }
//     pub fn new_transparent(nodes: Vec<Node>) -> Self
//     {
//         return Self {
//             nodes:          nodes,
//             is_transparent: true,
//             node_type:      Type::unknown(),
//         };
//     }
// }
// impl NodeRecur for SequenceNodeData
// {
//     fn recur_transformation<TParams>(
//         &mut self,
//         function: fn(&mut Node, &mut TParams),
//         params: &mut TParams,
//     )
//     {
//         for node in self.get_nodes_mut().iter_mut()
//         {
//             function(node, params);
//         }
//     }
//     fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
//     {
//         for node in self.get_nodes().iter()
//         {
//             function(node, params);
//         }
//     }
// }
// impl_typed!(SequenceNodeData);
// impl_to_node!(SequenceNodeData, Node::Sequence);

// #[derive(Clone)]
// pub struct ConditionalNodeData
// {
//     condition_node: OtherNode,
//     then_node:      OtherNode,
//     else_node:      OtherNode,
// }
// impl ConditionalNodeData
// {
//     get!(condition_node : get_condition -> &Node);
//     get!(condition_node : get_condition_mut -> &mut Node);
//     get!(then_node : get_then -> &Node);
//     get!(then_node : get_then_mut -> &mut Node);
//     get!(else_node : get_else -> &Node);
//     get!(else_node : get_else_mut -> &mut Node);

//     pub fn has_else(&self) -> bool
//     {
//         if let Node::Nothing = self.get_else()
//         {
//             return false;
//         }

//         return true;
//     }

//     pub fn new(condition_node: Node, then_node: Node, else_node: Node) -> Self
//     {
//         return Self {
//             condition_node: OtherNode::new(condition_node),
//             then_node:      OtherNode::new(then_node),
//             else_node:      OtherNode::new(else_node),
//         };
//     }
// }
// impl NodeRecur for ConditionalNodeData
// {
//     fn recur_transformation<TParams>(
//         &mut self,
//         function: fn(&mut Node, &mut TParams),
//         params: &mut TParams,
//     )
//     {
//         function(self.get_condition_mut(), params);
//         function(self.get_then_mut(), params);
//         function(self.get_else_mut(), params);
//     }
//     fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
//     {
//         function(self.get_condition(), params);
//         function(self.get_then(), params);
//         function(self.get_else(), params);
//     }
// }
// impl Typed for ConditionalNodeData
// {
//     fn get_type(&self) -> &Type
//     {
//         return self.get_then().get_type();
//     }
// }
// impl_to_node!(ConditionalNodeData, Node::Conditional);

// /* -------------------------------------------------------------------------- */
// /*                              Definition Nodes                              */
// /* -------------------------------------------------------------------------- */

// #[derive(Clone)]
// pub struct ArgumentData
// {
//     name:          String,
//     argument_type: Type,
// }
// impl ArgumentData
// {
//     get!(name : get_name -> &String);
//     get!(argument_type : get_type -> &Type);

//     pub fn new(name: String, argument_type: Type) -> Self
//     {
//         return Self {
//             name:          name,
//             argument_type: argument_type,
//         };
//     }
// }

// #[derive(Clone)]
// pub struct FunctionNodeData
// {
//     name:      String,
//     arguments: Vec<ArgumentData>,
//     body_node: OtherNode,

//     return_type: Type,
//     node_type:   Type,
// }
// impl FunctionNodeData
// {
//     get!(arguments : get_arguments -> &Vec<ArgumentData>);
//     get!(arguments : get_arguments_mut -> &mut Vec<ArgumentData>);
//     get!(body_node : get_body -> &Node);
//     get!(body_node : get_body_mut -> &mut Node);

//     get!(name : get_name -> &String);
//     get!(return_type: get_return_type -> &Type);

//     pub fn set_name(&mut self, new_name: String)
//     {
//         self.name = new_name;
//     }

//     pub fn new(
//         name: String,
//         arguments: Vec<ArgumentData>,
//         return_type: Type,
//         body: Node,
//         metadata: FunctionMetadata,
//     ) -> Self
//     {
//         // Build the function type based on node data
//         let mut argument_types = Vec::new();
//         for argument in arguments.iter()
//         {
//             argument_types.push(argument.get_type().clone());
//         }
//         let function_type = Type::from(FunctionTypeData::new(
//             argument_types,
//             return_type.clone(),
//             metadata,
//         ));

//         return Self {
//             name:        name,
//             arguments:   arguments,
//             body_node:   OtherNode::new(body),
//             node_type:   function_type,
//             return_type: return_type,
//         };
//     }
//     pub fn new_infer_type(
//         name: String,
//         arguments: Vec<ArgumentData>,
//         body: Node,
//         metadata: FunctionMetadata,
//     ) -> Self
//     {
//         return FunctionNodeData::new(name, arguments, body.get_type().clone(), body, metadata);
//     }
// }
// impl NodeRecur for FunctionNodeData
// {
//     fn recur_transformation<TParams>(
//         &mut self,
//         function: fn(&mut Node, &mut TParams),
//         params: &mut TParams,
//     )
//     {
//         function(self.get_body_mut(), params);
//     }
//     fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
//     {
//         function(self.get_body(), params);
//     }
// }
// impl_typed!(FunctionNodeData);
// impl_to_node!(FunctionNodeData, Node::Function);

// /* -------------------------------------------------------------------------- */
// /*                                 Structures                                 */
// /* -------------------------------------------------------------------------- */

// #[derive(Clone)]
// pub struct MemberData
// {
//     name:             String,
//     member_type:      Type,
//     read_visibility:  Visibility,
//     write_visibility: Visibility,
//     scope:            MemberScope,
// }
// impl MemberData
// {
//     get!(name : get_name -> &String);
//     get!(member_type : get_type -> &Type);

//     get!(read_visibility : get_read_visibility -> Visibility);
//     get!(write_visibility : get_write_visibility -> Visibility);

//     get!(scope : get_scope -> MemberScope);

//     pub fn set_read_visibility(&mut self, visibility: Visibility)
//     {
//         self.read_visibility = visibility;
//     }
//     pub fn set_write_visibility(&mut self, visibility: Visibility)
//     {
//         self.write_visibility = visibility;
//     }

//     pub fn new(
//         name: String,
//         member_type: Type,
//         read_visibility: Visibility,
//         write_visibility: Visibility,
//         scope: MemberScope,
//     ) -> Self
//     {
//         return Self {
//             name:             name,
//             member_type:      member_type,
//             read_visibility:  read_visibility,
//             write_visibility: write_visibility,
//             scope:            scope,
//         };
//     }
// }

// #[derive(Clone)]
// pub struct MethodData
// {
//     function_data: FunctionNodeData,
//     visibility:    Visibility,
//     scope:         MemberScope,
// }
// impl MethodData
// {
//     get!(function_data : get_function_data -> &FunctionNodeData);
//     get!(function_data : get_function_data_mut -> &mut FunctionNodeData);

//     get!(visibility : get_visibility -> Visibility);
//     get!(scope : get_scope -> MemberScope);

//     pub fn set_visibility(&mut self, visibility: Visibility)
//     {
//         self.visibility = visibility;
//     }

//     pub fn new(function_data: FunctionNodeData, visibility: Visibility, scope: MemberScope)
//         -> Self
//     {
//         return Self {
//             function_data: function_data,
//             visibility:    visibility,
//             scope:         scope,
//         };
//     }
// }
// #[derive(Clone)]
// pub struct TraitData
// {
//     name: String,
// }
// impl TraitData
// {
//     get!(name : get_name -> &String);

//     pub fn new(name: String) -> Self
//     {
//         return Self { name: name };
//     }
// }

// #[derive(Clone)]
// pub struct TypeNodeData
// {
//     name: String,

//     members: Vec<MemberData>,
//     methods: Vec<MethodData>,
//     traits:  Vec<TraitData>,

//     node_type: Type,
// }
// impl TypeNodeData
// {
//     get!(methods : get_methods -> &Vec<MethodData>);
//     get!(methods : get_methods_mut -> &mut Vec<MethodData>);

//     get!(members : get_members -> &Vec<MemberData>);
//     get!(members : get_members_mut -> &mut Vec<MemberData>);

//     get!(traits : get_traits -> &Vec<TraitData>);
//     get!(traits : get_traits_mut -> &mut Vec<TraitData>);

//     get!(name : get_name -> &String);
//     get!(name : get_name_mut -> &mut String);

//     pub fn get_instance_type(&self) -> Type
//     {
//         return Type::from(InstanceTypeData::new(self.name.clone()));
//     }

//     pub fn add_trait_name(&mut self, name: String)
//     {
//         self.traits.push(TraitData::new(name));
//     }

//     pub fn set_name(&mut self, name: String)
//     {
//         self.name = name;
//     }

//     pub fn new(name: String, members: Vec<MemberData>, methods: Vec<MethodData>, traits: Vec<TraitData>) -> Self
//     {
//         return Self {
//             name:      name,
//             members:   members,
//             methods:   methods,
//             traits:    Vec::new(),
//             node_type: Type::unknown(),
//         };
//     }
//     pub fn new_empty(name: String) -> Self
//     {
//         return Self {
//             name:      name,
//             members:   Vec::new(),
//             methods:   Vec::new(),
//             traits:    Vec::new(),
//             node_type: Type::unknown(),
//         };
//     }
// }

// impl NodeRecur for TypeNodeData
// {
//     fn recur_transformation<TParams>(
//         &mut self,
//         function: fn(&mut Node, &mut TParams),
//         params: &mut TParams,
//     )
//     {
//         for method in self.get_methods_mut().iter_mut()
//         {
//             function(method.get_function_data_mut().get_body_mut(), params);
//         }
//     }
//     fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
//     {
//         for method in self.get_methods().iter()
//         {
//             function(method.get_function_data().get_body(), params);
//         }
//     }
// }
// impl_typed!(TypeNodeData);
// impl_to_node!(TypeNodeData, Node::Type);

// #[derive(Clone)]
// pub struct AccessNodeData
// {
//     target:        OtherNode,
//     property_name: String,

//     node_type: Type,
// }
// impl AccessNodeData
// {
//     get!(target : get_target -> &Node);
//     get!(target : get_target_mut -> &mut Node);

//     get!(property_name : get_property -> &String);

//     pub fn new(target: Node, property_name: String) -> Self
//     {
//         return Self {
//             target:        OtherNode::new(target),
//             property_name: property_name,
//             node_type:     Type::unknown(),
//         };
//     }
// }
// impl NodeRecur for AccessNodeData
// {
//     fn recur_transformation<TParams>(
//         &mut self,
//         function: fn(&mut Node, &mut TParams),
//         params: &mut TParams,
//     )
//     {
//         function(self.get_target_mut(), params);
//     }
//     fn recur_parse<TParams>(&self, function: fn(&Node, &mut TParams), params: &mut TParams)
//     {
//         function(self.get_target(), params);
//     }
// }
// impl_typed!(AccessNodeData);
// impl_to_node!(AccessNodeData, Node::Access);