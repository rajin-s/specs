use super::internal::*;
use imports::*;

use super::parse_atomic;
use super::parse_function;
use super::parse_type;

use crate::language::ReferenceMode;
use crate::language::node::all::*;

pub enum ParseExpressionResult<'a>
{
    Complete(Node),
    Partial(PartialNodeData<'a>, Vec<ParseItemReference<'a>>),
    Error,
}

pub fn parse_expression<'a>(
    expression: &'a SExpression,
    context: &mut Context,
) -> ParseExpressionResult<'a>
{
    // Track all newly created child items
    let mut child_items = Vec::new();

    // Helper to create an Rc to an unparsed expression item and add a copy to the list of child items
    let mut child = |expression| {
        track_unparsed_child_item(expression, ParseItem::UnparsedNode, &mut child_items)
    };

    // Helper to create an Rc to a fully parsed expression
    //  note: this isn't tracked in child_items, since it doesn't need to be placed on the parse queue
    let complete_child = |node| Rc::new(ParseItem::CompleteNode(node));

    let partial_node_data = match expression
    {
        // symbol
        //  => Integer
        //  => Boolean
        //  => PrimitiveOperator
        //  => Variable
        Symbol(symbol) =>
        {
            if let Some(result) = parse_atomic::integer(symbol, context)
            {
                return ParseExpressionResult::Complete(Node::from(result));
            }
            else if let Some(result) = parse_atomic::boolean(symbol, context)
            {
                return ParseExpressionResult::Complete(Node::from(result));
            }
            else if let Some(result) = parse_atomic::primitive_operator(symbol, context)
            {
                return ParseExpressionResult::Complete(Node::from(result));
            }
            else if let Some(result) = parse_atomic::variable(symbol, context)
            {
                return ParseExpressionResult::Complete(Node::from(result));
            }
            else
            {
                context.add_error(
                    errors::FAILED_SYMBOL,
                    "Failed to parse symbol",
                    format!("{}", expression),
                );
                return ParseExpressionResult::Error;
            }
        }

        // (...)
        List(BracketType::Round, elements) => match elements.as_slice()
        {
            // Infix Access Operator
            //  (a . name)
            [a, Symbol(x), Symbol(name)] if x == operators::ACCESS =>
            {
                PartialNodeData::Access(child(a), name.clone())
            }
            // Infix Assignment Operator
            //  (a <- b)
            [a, Symbol(x), b] if x == operators::ASSIGN =>
            {
                PartialNodeData::Assignment(child(a), child(b))
            }
            // Infix Assignment Operator
            //  (b -> a)
            [b, Symbol(x), a] if x == operators::ASSIGN_REVERSE =>
            {
                PartialNodeData::Assignment(child(a), child(b))
            }
            // Infix Binary Operator
            //  (a ~ b)
            [a, Symbol(x), b] if operators::is_binary(x) =>
            {
                match parse_atomic::primitive_operator(x, context)
                {
                    Some(operator) =>
                    {
                        let operator = complete_child(Node::from(operator));
                        let operands = vec![child(a), child(b)];

                        PartialNodeData::Call(operator, operands)
                    }
                    None =>
                    {
                        context.add_error(
                            errors::INTERNAL,
                            "Unhandled infix binary operator",
                            expression,
                        );
                        return ParseExpressionResult::Error;
                    }
                }
            }
            // Unary Reference Operator
            //  (ref a)
            [Symbol(x), a] if x == operators::REFERENCE =>
            {
                PartialNodeData::Reference(ReferenceMode::Immutable, child(a))
            }
            // Unary Mutable Reference Operator
            //  (mut-ref a)
            [Symbol(x), a] if x == operators::MUTABLE_REFERENCE =>
            {
                PartialNodeData::Reference(ReferenceMode::Mutable, child(a))
            }
            // Unary Dereference Operator
            //  (deref a)
            [Symbol(x), a] if x == operators::DEREFERENCE => PartialNodeData::Dereference(child(a)),

            // Let Binding
            //  (let name = a)
            [Symbol(x1), Symbol(name), Symbol(x2), a]
                if x1 == keywords::BINDING && x2 == operators::ASSIGN_BINDING =>
            {
                PartialNodeData::Binding(name.clone(), child(a))
            }

            // If-Then
            //  (if a then b)
            [Symbol(x1), a, Symbol(x2), b] if x1 == keywords::IF && x2 == keywords::THEN =>
            {
                PartialNodeData::Conditional(child(a), child(b), complete_child(atomic::Nothing::new().to_node()))
            }
            // If-Then-Else
            //  (if a then b else c)
            [Symbol(x1), a, Symbol(x2), b, Symbol(x3), c]
                if x1 == keywords::IF && x2 == keywords::THEN && x3 == keywords::ELSE =>
            {
                PartialNodeData::Conditional(child(a), child(b), child(c))
            }
            // When
            //  (when {...})
            [Symbol(x1), List(BracketType::Curly, _inner_elements)] if x1 == keywords::WHEN =>
            {
                context.add_error(
                    errors::INTERNAL,
                    "when statement not implemented",
                    expression,
                );
                return ParseExpressionResult::Error;
            }
            // When-Else
            //  (when {...} else ...)
            [Symbol(x1), List(BracketType::Curly, _inner_elements), Symbol(x2), _a]
                if x1 == keywords::WHEN && x2 == keywords::ELSE =>
            {
                context.add_error(
                    errors::INTERNAL,
                    "when statement not implemented",
                    expression,
                );
                return ParseExpressionResult::Error;
            }

            elements =>
            {
                match elements.first()
                {
                    // Function
                    //  (fn ...)
                    Some(Symbol(x)) if x == keywords::FUNCTION && elements.len() >= 3 =>
                    {
                        match parse_function::definition(elements, false, context)
                        {
                            Some((data, _, new_child_items)) =>
                            {
                                let _: Vec<_> = new_child_items
                                    .into_iter()
                                    .map(|item| child_items.push(item))
                                    .collect();
                                PartialNodeData::Function(data)
                            }
                            None =>
                            {
                                context.add_error(
                                    errors::FAILED_NODE,
                                    "Failed to parse function definition",
                                    expression,
                                );
                                return ParseExpressionResult::Error;
                            }
                        }
                    }

                    // Type
                    //  (type ...)
                    Some(Symbol(x)) if x == keywords::TYPE && elements.len() >= 2 =>
                    {
                        match parse_type::definition(elements, context)
                        {
                            Some((data, new_child_items)) =>
                            {
                                let _: Vec<_> = new_child_items
                                    .into_iter()
                                    .map(|item| child_items.push(item))
                                    .collect();
                                PartialNodeData::Type(data)
                            }
                            None =>
                            {
                                context.add_error(
                                    errors::FAILED_NODE,
                                    "Failed to parse type definition",
                                    expression,
                                );
                                return ParseExpressionResult::Error;
                            }
                        }
                    }

                    // General Function Call
                    //  (a ...)
                    Some(a) =>
                    {
                        let operator = child(a);
                        let operands = elements.iter().skip(1).map(|x| child(x)).collect();

                        PartialNodeData::Call(operator, operands)
                    }

                    // Empty
                    //  ()
                    None =>
                    {
                        // For testing purposes, can be used to create an empty node
                        context.add_warning(
                            "EmptyNode",
                            "Empty nodes are only supported for testing",
                            "()",
                        );

                        return ParseExpressionResult::Complete(atomic::Nothing::new().to_node());
                    }
                }
            }
        },

        // {...}
        List(BracketType::Curly, elements) =>
        {
            let nodes = elements.iter().map(|x| child(x)).collect();
            PartialNodeData::Sequence(SequenceMode::Scope, nodes)
        }

        // [...]
        List(BracketType::Square, _elements) =>
        {
            context.add_error(errors::FAILED_NODE, "Unexpected [...] list", expression);
            return ParseExpressionResult::Error;
        }

        // <...>
        List(BracketType::None, elements) =>
        {
            let nodes = elements.iter().map(|x| child(x)).collect();
            PartialNodeData::Sequence(SequenceMode::Transparent, nodes)
        }
    };

    ParseExpressionResult::Partial(partial_node_data, child_items)
}

/* -------------------------------------------------------------------------- */
/*                          Partial => Complete Node                          */
/* -------------------------------------------------------------------------- */

fn get_node<'a>(item: ParseItemReference<'a>) -> Result<Node, ParseItemReference<'a>>
{
    match Rc::try_unwrap(item)
    {
        Ok(item) => match item
        {
            ParseItem::CompleteNode(value) => Ok(value),
            item => Err(Rc::new(item)),
        },
        Err(item) => Err(item),
    }
}
fn get_type<'a>(item: ParseItemReference<'a>) -> Result<Type, ParseItemReference<'a>>
{
    match Rc::try_unwrap(item)
    {
        Ok(item) => match item
        {
            ParseItem::CompleteType(value) => Ok(value),
            item => Err(Rc::new(item)),
        },
        Err(item) => Err(item),
    }
}

pub fn construct(data: PartialNodeData, context: &mut Context) -> Option<Node>
{
    use PartialNodeData::*;
    match data
    {
        Call(operator, operands) => construct_call_node(operator, operands, context),
        Reference(ref_type, target) => construct_reference_node(ref_type, target, context),
        Dereference(target) => construct_dereference_node(target, context),
        Binding(name, binding) => construct_binding_node(name, binding, context),
        Assignment(lhs, rhs) => construct_assignment_node(lhs, rhs, context),
        Sequence(mode, nodes) => construct_sequence_node(mode, nodes, context),
        Conditional(cond, thn, els) => construct_conditional_node(cond, thn, els, context),
        Function((name, args, ret, body)) =>
        {
            construct_function_node(name, args, ret, body, context)
        }
        Type((name, members, methods, traits)) =>
        {
            construct_type_node(name, members, methods, traits, context)
        }
        Access(target, name) => construct_access_node(target, name, context),

        _ =>
        {
            context.add_error(
                errors::FAILED_CONSTRUCT,
                "Trying to construct node from empty PartialNodeData",
                data,
            );
            None
        }
    }
}

fn construct_call_node(
    operator: ParseItemReference,
    operands: Vec<ParseItemReference>,
    context: &mut Context,
) -> Option<Node>
{
    match get_node(operator)
    {
        Ok(operator) =>
        {
            let operand_count = operands.len();

            // Convert to a list of nodes
            let operands: Vec<_> = operands
                .into_iter()
                .filter_map(|x| match get_node(x)
                {
                    Ok(n) => Some(n),
                    _ => None,
                })
                .collect();

            // Make sure all nodes were successfully converted
            if operands.len() == operand_count
            {
                Some(operator::Call::new(operator, operands).to_node())
            }
            else
            {
                context.add_error(
                    errors::FAILED_CONSTRUCT,
                    "Failed to get complete nodes for call operands",
                    format!("failed {} nodes", operand_count - operands.len()),
                );
                None
            }
        }
        Err(item) =>
        {
            context.add_error(
                errors::FAILED_CONSTRUCT,
                "Failed to get complete node for call operator",
                item.as_ref(),
            );
            None
        }
    }
}

fn construct_reference_node<'a>(
    mode: ReferenceMode,
    target: ParseItemReference<'a>,
    context: &mut Context,
) -> Option<Node>
{
    match get_node(target)
    {
        Ok(target) => Some(operator::Reference::new(mode, target).to_node()),
        Err(item) =>
        {
            context.add_error(
                errors::FAILED_CONSTRUCT,
                "Failed to get complete node for reference target",
                item.as_ref(),
            );
            None
        }
    }
}
fn construct_dereference_node<'a>(
    target: ParseItemReference<'a>,
    context: &mut Context,
) -> Option<Node>
{
    match get_node(target)
    {
        Ok(target) => Some(operator::Dereference::new(target).to_node()),
        Err(item) =>
        {
            context.add_error(
                errors::FAILED_CONSTRUCT,
                "Failed to get complete node for dereference target",
                item.as_ref(),
            );
            None
        }
    }
}

fn construct_binding_node<'a>(
    name: String,
    binding: ParseItemReference<'a>,
    context: &mut Context,
) -> Option<Node>
{
    match get_node(binding)
    {
        Ok(binding) => Some(binding::Binding::new(name, binding).to_node()),
        Err(item) =>
        {
            context.add_error(
                errors::FAILED_CONSTRUCT,
                "Failed to get complete node for reference target",
                item.as_ref(),
            );
            None
        }
    }
}
fn construct_assignment_node<'a>(
    lhs: ParseItemReference<'a>,
    rhs: ParseItemReference<'a>,
    context: &mut Context,
) -> Option<Node>
{
    match (get_node(lhs), get_node(rhs))
    {
        (Ok(lhs), Ok(rhs)) => Some(operator::Assign::new(lhs, rhs).to_node()),
        (lhs, rhs) =>
        {
            if let Err(item) = lhs
            {
                context.add_error(
                    errors::FAILED_CONSTRUCT,
                    "Failed to get complete node for assignment LHS",
                    item.as_ref(),
                );
            }
            if let Err(item) = rhs
            {
                context.add_error(
                    errors::FAILED_CONSTRUCT,
                    "Failed to get complete node for assignment RHS",
                    item.as_ref(),
                );
            }
            None
        }
    }
}
fn construct_sequence_node<'a>(
    mode: control::SequenceMode,
    nodes: Vec<ParseItemReference<'a>>,
    context: &mut Context,
) -> Option<Node>
{
    let node_count = nodes.len();

    // Convert to a list of nodes
    let nodes: Vec<_> = nodes
        .into_iter()
        .filter_map(|x| match get_node(x)
        {
            Ok(n) => Some(n),
            _ => None,
        })
        .collect();

    // Make sure all nodes were successfully converted
    if nodes.len() == node_count
    {
        Some(control::Sequence::new(mode, nodes).to_node())
    }
    else
    {
        context.add_error(
            errors::FAILED_CONSTRUCT,
            "Failed to get complete nodes for sequence",
            format!("failed {} nodes", node_count - nodes.len()),
        );
        None
    }
}
fn construct_conditional_node<'a>(
    condition: ParseItemReference<'a>,
    then_branch: ParseItemReference<'a>,
    else_branch: ParseItemReference<'a>,
    context: &mut Context,
) -> Option<Node>
{
    match (
        get_node(condition),
        get_node(then_branch),
        get_node(else_branch),
    )
    {
        (Ok(condition), Ok(then_branch), Ok(else_branch)) =>
        {
            Some(control::Conditional::new(condition, then_branch, else_branch).to_node())
        }
        (condition, then_branch, else_branch) =>
        {
            if let Err(item) = condition
            {
                context.add_error(
                    errors::FAILED_CONSTRUCT,
                    "Failed to get complete node for if condition",
                    item.as_ref(),
                );
            }
            if let Err(item) = then_branch
            {
                context.add_error(
                    errors::FAILED_CONSTRUCT,
                    "Failed to get complete node for if then-branch",
                    item.as_ref(),
                );
            }
            if let Err(item) = else_branch
            {
                context.add_error(
                    errors::FAILED_CONSTRUCT,
                    "Failed to get complete node for if else-branch",
                    item.as_ref(),
                );
            }
            None
        }
    }
}

fn construct_function_node_data<'a>(
    name: String,
    arguments: Vec<PartialArgumentData<'a>>,
    return_type: ParseItemReference<'a>,
    body: ParseItemReference<'a>,
    context: &mut Context,
) -> Option<definition::Function>
{
    match (get_node(body), get_type(return_type))
    {
        (Ok(body), Ok(return_type)) =>
        {
            let argument_count = arguments.len();
            let arguments: Vec<_> = arguments
                .into_iter()
                .filter_map(|(name, t)| match get_type(t)
                {
                    Ok(t) => Some(definition::Argument::new(name, t)),
                    Err(item) =>
                    {
                        context.add_error(
                            errors::FAILED_CONSTRUCT,
                            "Failed to get complete type for function argument",
                            item,
                        );
                        None
                    }
                })
                .collect();

            if arguments.len() == argument_count
            {
                Some(definition::Function::new(
                    name,
                    arguments,
                    return_type,
                    body,
                ))
            }
            else
            {
                context.add_error(
                    errors::FAILED_CONSTRUCT,
                    "Failed to get construct all function arguments",
                    format!("failed {} arguments", argument_count - arguments.len()),
                );
                None
            }
        }
        (body, return_type) =>
        {
            if let Err(item) = body
            {
                context.add_error(
                    errors::FAILED_CONSTRUCT,
                    "Failed to get complete node for function body",
                    item.as_ref(),
                );
            }
            if let Err(item) = return_type
            {
                context.add_error(
                    errors::FAILED_CONSTRUCT,
                    "Failed to get complete node for function return type",
                    item.as_ref(),
                );
            }
            None
        }
    }
}

fn construct_function_node<'a>(
    name: String,
    arguments: Vec<PartialArgumentData<'a>>,
    return_type: ParseItemReference<'a>,
    body: ParseItemReference<'a>,
    context: &mut Context,
) -> Option<Node>
{
    match construct_function_node_data(name, arguments, return_type, body, context)
    {
        Some(data) => Some(Node::from(data)),
        None => None,
    }
}

fn construct_type_node<'a>(
    _name: String,
    _members: Vec<PartialMemberData<'a>>,
    _methods: Vec<PartialMethodData<'a>>,
    _traits: Vec<String>,
    _context: &mut Context,
) -> Option<Node>
{
    return None;

    // let member_count = members.len();
    // let method_count = methods.len();

    // let members: Vec<_> = members
    //     .into_iter()
    //     .filter_map(|(name, t, scope, read, write)| match get_type(t)
    //     {
    //         Ok(t) => Some(MemberData::new(name, t, read, write, scope)),
    //         Err(item) =>
    //         {
    //             context.add_error(
    //                 errors::FAILED_CONSTRUCT,
    //                 "Failed to get complete type for type definition member",
    //                 item,
    //             );
    //             None
    //         }
    //     })
    //     .collect();

    // let methods: Vec<_> = methods
    //     .into_iter()
    //     .filter_map(
    //         |((name, arguments, return_type, body), scope, visibility)| {
    //             match construct_function_node_data(name, arguments, return_type, body, context)
    //             {
    //                 Some(function_data) => Some(MethodData::new(function_data, visibility, scope)),
    //                 None => None,
    //             }
    //         },
    //     )
    //     .collect();

    // let traits: Vec<_> = traits
    //     .into_iter()
    //     .map(|name| TraitData::new(name))
    //     .collect();

    // // Make all members and methods were constructed
    // match (member_count - members.len(), method_count - methods.len())
    // {
    //     (0, 0) => Some(Node::from(TypeNodeData::new(
    //         name, members, methods, traits,
    //     ))),
    //     (failed_members, failed_methods) =>
    //     {
    //         context.add_error(
    //             errors::FAILED_CONSTRUCT,
    //             "Failed to get construct all function arguments",
    //             format!(
    //                 "failed {} members, {} methods",
    //                 failed_members, failed_methods
    //             ),
    //         );
    //         None
    //     }
    // }
}

fn construct_access_node<'a>(
    target: ParseItemReference<'a>,
    name: String,
    context: &mut Context,
) -> Option<Node>
{
    match get_node(target)
    {
        Ok(target) => Some(operator::Access::new(target, name).to_node()),
        Err(item) =>
        {
            context.add_error(
                errors::FAILED_CONSTRUCT,
                "Failed to get complete node for reference target",
                item.as_ref(),
            );
            None
        }
    }
}
