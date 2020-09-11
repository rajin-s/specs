use crate::utilities::*;

pub use crate::language::node::*;
pub use crate::language::s_expression::*;
pub use crate::language::symbols::*;

pub use crate::errors::parse_error::*;
pub use crate::source::Source;

impl super::Parser
{
    ///
    /// Try to create a Node from an SExpression
    ///  
    pub fn make_node(&self, s_expression: SExpression) -> ResultLog<Node, Error>
    {
        // Create an unparsed root node

        let mut root = ParseNode::UnparsedNode(s_expression);

        // Create and apply a parse transformation

        let mut transform = ParseTransform::new();
        let mut warnings = Vec::new();

        match transform.apply(&mut root)
        {
            ResultLog::Ok(()) => (),
            ResultLog::Warn((), mut new_warnings) => warnings.append(&mut new_warnings),
            ResultLog::Error(errors, warnings) => return ResultLog::Error(errors, warnings),
        }

        // Make sure we actually ended up with a parsed node

        match root
        {
            ParseNode::ParsedNode(node) => ResultLog::maybe_warn(node, warnings),
            _ =>
            {
                let error = Error::Internal(format!("Failed to parse root node: {:?}", root));
                ResultLog::Error(vec![error], warnings)
            }
        }
    }
}

///
/// Recursive parse tree structure
///
#[derive(Debug)]
pub enum ParseNode
{
    ParsedNode(Node),
    UnparsedNode(SExpression),
    PartialNode(NodeKind, PartialNodeData, Vec<ParseNode>, Source),

    ParsedType(Type),
    UnparsedType(SExpression),
    PartialType(Vec<ParseNode>, Source),
}

impl ParseNode
{
    pub fn get_source(&self) -> Source
    {
        match self
        {
            ParseNode::ParsedNode(node) => node.get_source(),
            ParseNode::UnparsedNode(s_expression) => s_expression.get_source(),
            ParseNode::PartialNode(_, _, _, source) => source.clone(),

            ParseNode::ParsedType(_) => Source::empty(),
            ParseNode::UnparsedType(s_expression) => s_expression.get_source(),
            ParseNode::PartialType(_, source) => source.clone(),
        }
    }
}

impl Recur<ParseNode> for ParseNode
{
    fn get_children(&self) -> Vec<&ParseNode>
    {
        match self
        {
            ParseNode::PartialNode(
                NodeKind::Function,
                PartialNodeData::Function(_, _, _, types),
                children,
                _,
            ) =>
            {
                // Make sure type annotations are added as child nodes
                children.iter().chain(types.iter()).collect()
            }
            ParseNode::PartialNode(_, _, children, _) => children.iter().collect(),
            _ => Vec::new(),
        }
    }
    fn get_children_mut(&mut self) -> Vec<&mut ParseNode>
    {
        match self
        {
            ParseNode::PartialNode(
                NodeKind::Function,
                PartialNodeData::Function(_, _, _, types),
                children,
                _,
            ) => children.iter_mut().chain(types.iter_mut()).collect(),
            ParseNode::PartialNode(_, _, children, _) => children.iter_mut().collect(),
            _ => Vec::new(),
        }
    }
}

///
/// Data specific to a particular kind of node, potentially including more child ParseNodes (for types)
///
#[derive(Debug)]
pub enum PartialNodeData
{
    None,
    Access(String),
    Binding(String),
    Sequence(SequenceMode),
    Reference(ReferenceMode),
    Function(String, bool, Vec<String>, Vec<ParseNode>),
}

impl Default for PartialNodeData
{
    fn default() -> Self
    {
        PartialNodeData::None
    }
}

///
/// Parse transformation requires no state
/// 
struct ParseState {}

///
/// Recursive parse transformation
///
struct ParseTransform {}

impl ParseTransform
{
    pub fn new() -> ParseTransform
    {
        ParseTransform {}
    }
}

impl RecurTransform<ParseNode, ParseState, Error> for ParseTransform
{
    fn get_root_state(&mut self, _root: &ParseNode) -> ParseState
    {
        ParseState {}
    }

    fn enter(&mut self, node: &mut ParseNode, _state: &mut ParseState) -> ResultLog<(), Error>
    {
        // On entering a ParseNode, look at its structure and track all information needed to
        //  construct a completed Node / Type once all child items are parsed

        match node
        {
            ParseNode::ParsedNode(_) => ResultLog::Ok(()),
            ParseNode::UnparsedNode(s_expression) =>
            {
                let original_expression = SExpression::take(s_expression);

                match make_partial_node(original_expression)
                {
                    ResultLog::Ok(new_node) =>
                    {
                        *node = new_node;
                        ResultLog::Ok(())
                    }
                    ResultLog::Warn(new_node, warnings) =>
                    {
                        *node = new_node;
                        ResultLog::Warn((), warnings)
                    }
                    ResultLog::Error(errors, warnings) => ResultLog::Error(errors, warnings),
                }
            }
            ParseNode::ParsedType(_) => ResultLog::Ok(()),
            ParseNode::UnparsedType(s_expression) =>
            {
                let original_expression = SExpression::take(s_expression);

                match make_partial_type(original_expression)
                {
                    ResultLog::Ok(new_node) =>
                    {
                        *node = new_node;
                        ResultLog::Ok(())
                    }
                    ResultLog::Warn(new_node, warnings) =>
                    {
                        *node = new_node;
                        ResultLog::Warn((), warnings)
                    }
                    ResultLog::Error(errors, warnings) => ResultLog::Error(errors, warnings),
                }
            }
            ParseNode::PartialNode(..) | ParseNode::PartialType(..) =>
            {
                ResultLog::new_error(Error::Internal(format!("Unexpected enter: {:?}", node)))
            }
        }
    }

    fn exit(&mut self, node: &mut ParseNode, _state: &mut ParseState) -> ResultLog<(), Error>
    {
        // On exiting a ParseNode, all child items have been parsed so we should be ready to
        //  create a completed Node or Type

        match node
        {
            ParseNode::ParsedNode(..) => ResultLog::Ok(()),
            ParseNode::PartialNode(kind, data, children, source) =>
            {
                // Turn a partial node into a completed Node

                let original_children = std::mem::take(children);
                let mut new_children = Vec::with_capacity(original_children.len());
                let mut has_all_children = true;

                // Ensure all child items were fully parsed

                for child in original_children
                {
                    match child
                    {
                        ParseNode::ParsedNode(node) =>
                        {
                            new_children.push(node);
                        }
                        _ =>
                        {
                            has_all_children = false;
                        }
                    }
                }

                if has_all_children
                {
                    match make_complete_node(
                        *kind,
                        std::mem::take(data),
                        new_children,
                        source.clone(),
                    )
                    {
                        ResultLog::Ok(new_node) =>
                        {
                            *node = ParseNode::ParsedNode(new_node);
                            ResultLog::Ok(())
                        }
                        ResultLog::Warn(new_node, warnings) =>
                        {
                            *node = ParseNode::ParsedNode(new_node);
                            ResultLog::Warn((), warnings)
                        }
                        ResultLog::Error(errors, warnings) => ResultLog::Error(errors, warnings),
                    }
                }
                else
                {
                    ResultLog::new_error(Error::Internal(format!(
                        "Failed to parse all children: {:?}",
                        node
                    )))
                }
            }
            ParseNode::ParsedType(..) => ResultLog::Ok(()),
            ParseNode::PartialType(..) => unimplemented!(), // Currently, we're only handling types that have no child ParseNodes

            ParseNode::UnparsedNode(..) | ParseNode::UnparsedType(..) =>
            {
                ResultLog::new_error(Error::Internal(format!("Unexpected exit: {:?}", node)))
            }
        }
    }
}

use super::parse_atomic;
use super::parse_function;

///
/// Inspect the structure of a potential Node and create a partial or completed ParseNode
///
fn make_partial_node(s_expression: SExpression) -> ResultLog<ParseNode, Error>
{
    use SExpression::*;

    fn unparsed(s_expression: &mut SExpression) -> ParseNode
    {
        ParseNode::UnparsedNode(SExpression::take(s_expression))
    }
    fn parsed<T: ToNode>(data: T) -> ParseNode
    {
        ParseNode::ParsedNode(data.to_node())
    }

    let mut warnings = Vec::new();

    match s_expression
    {
        Symbol(symbol, source) =>
        {
            // Symbols can be immediately parsed into atomic nodes

            match parse_atomic::integer(&symbol, &source)
            {
                Some(integer) => return ResultLog::Ok(parsed(integer)),
                None => (),
            }
            match parse_atomic::boolean(&symbol, &source)
            {
                Some(boolean) => return ResultLog::Ok(parsed(boolean)),
                None => (),
            }
            match parse_atomic::primitive_operator(&symbol, &source)
            {
                Some(operator) => return ResultLog::Ok(parsed(operator)),
                None => (),
            }
            match parse_atomic::variable(symbol, source)
            {
                ResultLog::Ok(variable) => return ResultLog::Ok(parsed(variable)),
                ResultLog::Warn(variable, warnings) =>
                {
                    return ResultLog::Warn(parsed(variable), warnings)
                }
                ResultLog::Error(errors, warnings) => return ResultLog::Error(errors, warnings),
            }
        }
        List(BracketType::Round, mut elements, source) =>
        {
            // (...) lists

            let (node_kind, node_data, children) = match elements.as_mut_slice()
            {
                // Infix Access Operator
                //  (a . name)
                [a, Symbol(x, _), Symbol(name, _)] if x == operators::ACCESS => (
                    NodeKind::Access,
                    PartialNodeData::Access(std::mem::take(name)),
                    vec![unparsed(a)],
                ),
                // Infix Assignment Operator
                //  (a <- b)
                [a, Symbol(x, _), b] if x == operators::ASSIGN => (
                    NodeKind::Assign,
                    PartialNodeData::None,
                    vec![unparsed(a), unparsed(b)],
                ),
                // Infix Assignment Operator
                //  (b -> a)
                [b, Symbol(x, _), a] if x == operators::ASSIGN_REVERSE => (
                    NodeKind::Assign,
                    PartialNodeData::None,
                    vec![unparsed(a), unparsed(b)],
                ),
                // Infix Binary Operator
                //  (a ~ b)
                [a, Symbol(x, x_source), b] if operators::is_binary(x) =>
                {
                    match parse_atomic::primitive_operator(x, x_source)
                    {
                        Some(operator) => (
                            NodeKind::Call,
                            PartialNodeData::None,
                            vec![parsed(operator), unparsed(a), unparsed(b)],
                        ),
                        None =>
                        {
                            let error =
                                Error::Internal(format!("Unhandled binary operator: '{}'", x));
                            return ResultLog::new_error(error);
                        }
                    }
                }
                // Unary Reference Operator
                //  (ref a)
                [Symbol(x, _), a] if x == operators::REFERENCE => (
                    NodeKind::Reference,
                    PartialNodeData::Reference(ReferenceMode::Immutable),
                    vec![unparsed(a)],
                ),
                // Unary Mutable Reference Operator
                //  (mut-ref a)
                [Symbol(x, _), a] if x == operators::MUTABLE_REFERENCE => (
                    NodeKind::Reference,
                    PartialNodeData::Reference(ReferenceMode::Mutable),
                    vec![unparsed(a)],
                ),
                // Unary Dereference Operator
                //  (deref a)
                [Symbol(x, _), a] if x == operators::DEREFERENCE => (
                    NodeKind::Dereference,
                    PartialNodeData::None,
                    vec![unparsed(a)],
                ),
                // Let Binding
                //  (let name = a)
                [Symbol(x1, _), Symbol(name, _), Symbol(x2, _), a]
                    if x1 == keywords::BINDING && x2 == operators::ASSIGN_BINDING =>
                {
                    (
                        NodeKind::Binding,
                        PartialNodeData::Binding(std::mem::take(name)),
                        vec![unparsed(a)],
                    )
                }
                // If-Then
                //  (if a then b)
                [Symbol(x1, _), a, Symbol(x2, _), b]
                    if x1 == keywords::IF && x2 == keywords::THEN =>
                {
                    (
                        NodeKind::Conditional,
                        PartialNodeData::None,
                        vec![unparsed(a), unparsed(b)],
                    )
                }
                // If-Then-Else
                //  (if a then b else c)
                [Symbol(x1, _), a, Symbol(x2, _), b, Symbol(x3, _), c]
                    if x1 == keywords::IF && x2 == keywords::THEN && x3 == keywords::ELSE =>
                {
                    (
                        NodeKind::Conditional,
                        PartialNodeData::None,
                        vec![unparsed(a), unparsed(b), unparsed(c)],
                    )
                }
                // (...)
                s_expressions =>
                {
                    match s_expressions.first()
                    {
                        // Function
                        //  (fn ...)
                        Some(Symbol(x, _))
                            if x == keywords::FUNCTION && s_expressions.len() >= 3 =>
                        {
                            match parse_function::definition(
                                elements,
                                parse_function::Mode::StaticOnly,
                                &source,
                            )
                            {
                                ResultLog::Ok(parts) => parts,
                                ResultLog::Warn(parts, mut new_warnings) =>
                                {
                                    warnings.append(&mut new_warnings);
                                    parts
                                }
                                ResultLog::Error(errors, mut new_warnings) =>
                                {
                                    warnings.append(&mut new_warnings);
                                    return ResultLog::Error(errors, warnings);
                                }
                            }
                        }
                        // Class
                        //  (type ...)
                        Some(Symbol(x, _)) if x == keywords::TYPE && s_expressions.len() >= 3 =>
                        {
                            let error = Error::Internal(format!("Classes not yet implemented"));
                            return ResultLog::new_error(error);
                        }
                        // Call
                        //  (...)
                        Some(_) => (
                            NodeKind::Call,
                            PartialNodeData::None,
                            elements.into_iter().map(ParseNode::UnparsedNode).collect(),
                        ),
                        // Nothing
                        //  ()
                        None =>
                        {
                            return ResultLog::Ok(parsed(Nothing::new(source)));
                        }
                    }
                }
                _ =>
                {
                    return ResultLog::new_error(Error::UnknownExpression(
                        format!("{}", &List(BracketType::Square, elements, source.clone())),
                        source,
                    ));
                }
            };

            ResultLog::Ok(ParseNode::PartialNode(
                node_kind, node_data, children, source,
            ))
        }
        List(BracketType::Curly, elements, source) =>
        {
            // {...} lists

            ResultLog::Ok(ParseNode::PartialNode(
                NodeKind::Sequence,
                PartialNodeData::Sequence(SequenceMode::Scope),
                elements.into_iter().map(ParseNode::UnparsedNode).collect(),
                source,
            ))
        }
        List(BracketType::Square, elements, source) =>
        {
            // [...] lists

            ResultLog::new_error(Error::UnknownExpression(
                format!("{}", &List(BracketType::Square, elements, source.clone())),
                source,
            ))
        }
        List(BracketType::None, elements, source) =>
        {
            // <...> lists (internally generated)

            ResultLog::new_error(Error::UnknownExpression(
                format!("{}", &List(BracketType::None, elements, source.clone())),
                source,
            ))
        }
        Empty(_) => ResultLog::new_error(Error::Internal(format!("Unexpected empty expression"))),
    }
}

///
/// Take the data from a partial node and try to create a completed Node
///
fn make_complete_node(
    kind: NodeKind,
    data: PartialNodeData,
    children: Vec<Node>,
    source: Source,
) -> ResultLog<Node, Error>
{
    let node = match (kind, data, children.len())
    {
        (NodeKind::Call, PartialNodeData::None, n) if n > 0 =>
        {
            let mut children_iter = children.into_iter();
            let operator = children_iter.next().unwrap();

            Call::new(operator, children_iter.collect(), source).to_node()
        }
        (NodeKind::Reference, PartialNodeData::Reference(mode), 1) =>
        {
            let a = children.into_1();
            Reference::new(mode, a, source).to_node()
        }
        (NodeKind::Dereference, PartialNodeData::None, 1) =>
        {
            let a = children.into_1();
            Dereference::new(a, source).to_node()
        }
        (NodeKind::Assign, PartialNodeData::None, 2) =>
        {
            let (a, b) = children.into_2();
            Assign::new(a, b, source).to_node()
        }
        (NodeKind::Access, PartialNodeData::Access(name), 1) =>
        {
            let a = children.into_1();
            Access::new(a, name, source).to_node()
        }

        (NodeKind::Binding, PartialNodeData::Binding(name), 1) =>
        {
            let a = children.into_1();
            Binding::new(name, a, source).to_node()
        }

        (NodeKind::Sequence, PartialNodeData::Sequence(sequence_mode), _) =>
        {
            Sequence::new(sequence_mode, children, source).to_node()
        }
        (NodeKind::Conditional, PartialNodeData::None, 2) =>
        {
            let (a, b) = children.into_2();
            Conditional::new(a, b, Node::nothing(source.clone()), source.clone()).to_node()
        }
        (NodeKind::Conditional, PartialNodeData::None, 3) =>
        {
            let (a, b, c) = children.into_3();
            Conditional::new(a, b, c, source.clone()).to_node()
        }

        (NodeKind::Function, PartialNodeData::Function(name, false, argument_names, types), 1) =>
        {
            let mut types_iter = types.into_iter();
            // Make sure we have a fully-parsed return type

            let return_type = match types_iter
                .next()
                .expect("Failed to get partial function return type")
            {
                ParseNode::ParsedType(t) => t,
                node =>
                {
                    return ResultLog::new_error(Error::Internal(format!(
                        "Failed to parse return type: {:?}",
                        node
                    )));
                }
            };

            // Make sure we have types for all arguments

            if types_iter.len() != argument_names.len()
            {
                return ResultLog::new_error(Error::Internal(format!(
                    "Number of argument ({}) names doesn't match number of types ({})",
                    argument_names.len(),
                    types_iter.len()
                )));
            }

            let mut arguments = Vec::with_capacity(argument_names.len());

            for (name, type_node) in argument_names.into_iter().zip(types_iter)
            {
                match type_node
                {
                    ParseNode::ParsedType(t) => arguments.push(Argument::new(name, t)),
                    node =>
                    {
                        return ResultLog::new_error(Error::Internal(format!(
                            "Failed to parse argument type: {:?}",
                            node
                        )));
                    }
                }
            }

            let body = children.into_1();
            Function::new(name, arguments, return_type, body, source).to_node()
        }

        (kind, mode, len) =>
        {
            return ResultLog::new_error(Error::Internal(format!(
                "Unexpected partial node: {:?} (mode: {:?}, children: {})",
                kind, mode, len
            )));
        }
    };

    ResultLog::Ok(node)
}

///
/// Inspect the structure of a potential Type and create a partial or completed ParseNode
///
fn make_partial_type(s_expression: SExpression) -> ResultLog<ParseNode, Error>
{
    use SExpression::*;

    match s_expression
    {
        Symbol(symbol, _) =>
        {
            let t = match symbol.as_str()
            {
                primitive_data_types::INTEGER => Type::Integer,
                primitive_data_types::BOOLEAN => Type::Boolean,
                primitive_data_types::FLOAT => Type::Float,
                primitive_data_types::VOID => Type::Void,

                // Any non-primitive types identified with a symbol are instances
                s => InstanceType::new(String::from(s)).to_type(),
            };

            ResultLog::Ok(ParseNode::ParsedType(t))
        }
        s_expression => ResultLog::new_error(Error::UnknownExpression(
            format!("Unknown type expression: {}", s_expression),
            s_expression.get_source(),
        )),
    }
}
