use super::internal::*;
use imports::*;

use crate::language::{MemberScope, Visibility};

use super::parse_function;

/* -------------------------------------------------------------------------- */
/*                         Parse Type from Expression                         */
/* -------------------------------------------------------------------------- */

pub enum ParseExpressionResult<'a>
{
    Complete(Type),
    Partial(PartialTypeData<'a>, Vec<ParseItemReference<'a>>),
    Error,
}

pub fn parse_expression<'a>(
    expression: &'a SExpression,
    context: &mut Context,
) -> ParseExpressionResult<'a>
{
    // let mut child_items = Vec::new();

    match expression
    {
        Symbol(symbol) =>
        {
            let t = match symbol.as_str()
            {
                primitive_data_types::INTEGER => basic_types::integer(),
                primitive_data_types::BOOLEAN => basic_types::boolean(),
                primitive_data_types::VOID => basic_types::void(),
                _ => class::InstanceType::new(symbol.clone()).to_type(),
            };

            ParseExpressionResult::Complete(t)
        }

        List(BracketType::Round, elements) =>
        {
            let _partial = match elements.as_slice()
            {
                _ =>
                {
                    context.add_error(
                        errors::FAILED_TYPE,
                        "Unexpected (...) type expression",
                        expression,
                    );
                    return ParseExpressionResult::Error;
                }
            };

            // ParseExpressionResult::Partial(partial, child_items)
        }

        _ =>
        {
            context.add_error(
                errors::FAILED_TYPE,
                "Unexpected type expression",
                expression,
            );
            return ParseExpressionResult::Error;
        }
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

pub fn construct(data: PartialTypeData, context: &mut Context) -> Option<Type>
{
    context.add_error(errors::FAILED_CONSTRUCT, "Failed to construct type", data);
    None
}

/* -------------------------------------------------------------------------- */
/*                            Parse Type Definition                           */
/* -------------------------------------------------------------------------- */

pub fn definition<'a>(
    elements: &'a [SExpression],
    context: &mut Context,
) -> Option<(PartialTypeDefinitionData<'a>, Vec<ParseItemReference<'a>>)>
{
    use std::collections::HashSet;

    // note: 'type' keywords already matched
    match elements
    {
        [_, Symbol(name), List(BracketType::Curly, elements)] =>
        {
            // Track all newly created child items
            let mut child_items = Vec::new();

            // Accumulate members and methods from blocks
            let mut members = Vec::new();
            let mut methods = Vec::new();

            let mut traits: Vec<String> = Vec::new();

            // Public read/write declarations can come before member declarations
            // so keep track of access specifiers and apply after all blocks are read
            let mut public_read = HashSet::new();
            let mut public_write = HashSet::new();

            for element in elements
            {
                match element
                {
                    // <...>
                    List(BracketType::None, group_elements) =>
                    {
                        match group_elements.as_slice()
                        {
                            // <data {...}>
                            [Symbol(x), List(BracketType::Curly, block_elements)]
                                if x == keywords::TYPE_DATA =>
                            {
                                let (mut new_members, mut new_child_items) =
                                    parse_data_block(block_elements, context);

                                // Extract all data from the block
                                members.append(&mut new_members);
                                child_items.append(&mut new_child_items);
                            }
                            // <private/public {...}>
                            [Symbol(x), List(BracketType::Curly, block_elements)]
                                if x == keywords::PRIVATE || x == keywords::PUBLIC =>
                            {
                                let visibility = match x == keywords::PUBLIC
                                {
                                    true => Visibility::Public,
                                    false => Visibility::Private,
                                };

                                let (
                                    mut new_methods,
                                    new_public_read,
                                    new_public_write,
                                    mut new_child_items,
                                ) = parse_method_block(block_elements, visibility, context);

                                // Collect method definitions
                                methods.append(&mut new_methods);

                                // Collect public read/write access specifiers
                                let _: Vec<_> = new_public_read
                                    .into_iter()
                                    .map(|x| public_read.insert(x))
                                    .collect();
                                let _: Vec<_> = new_public_write
                                    .into_iter()
                                    .map(|x| public_write.insert(x))
                                    .collect();

                                // Collect new child items
                                child_items.append(&mut new_child_items);
                            }
                            // <is T>
                            [Symbol(x), Symbol(name)] if x == keywords::IS =>
                            {
                                traits.push(name.clone());
                            }
                            // <is T {}>
                            [Symbol(x), Symbol(name), List(BracketType::Curly, _block_elements)]
                                if x == keywords::IS =>
                            {
                                traits.push(name.clone());
                                context.add_warning(
                                    errors::INTERNAL,
                                    "trait implementions not yet implemented",
                                    element,
                                );
                            }

                            _ =>
                            {
                                context.add_error(
                                    errors::FAILED_TYPE_DEFINITION,
                                    "Unexpected <...> list in type definition",
                                    element,
                                );
                            }
                        }
                    }

                    // The only valid elements are <...> lists that were created by the preprocessor
                    _ =>
                    {
                        context.add_error(
                            errors::FAILED_TYPE_DEFINITION,
                            "Unexpected list in type definition",
                            element,
                        );
                    }
                }
            }

            // Apply access specifiers to members
            for (name, _, scope, read, write) in members.iter_mut()
            {
                let is_instance_member = *scope == MemberScope::Instance;
                let pair = (name.clone(), is_instance_member);

                if public_read.contains(&pair)
                {
                    *read = Visibility::Public;
                }
                if public_write.contains(&pair)
                {
                    *read = Visibility::Public;
                    *write = Visibility::Public;
                }
            }

            let type_definition_data = (name.clone(), members, methods, traits);
            Some((type_definition_data, child_items))
        }

        _ =>
        {
            context.add_error(
                errors::FAILED_TYPE_DEFINITION,
                "Invalid type definition layout",
                List(BracketType::Round, elements.to_vec()),
            );
            None
        }
    }
}

// Parse either 'name' or '(self . name)'
fn parse_scoped_name(
    expression: &SExpression,
    context: &mut Context,
) -> Option<(String, MemberScope)>
{
    match expression
    {
        Symbol(name) => Some((name.clone(), MemberScope::Static)),
        List(BracketType::Round, elements) => match elements.as_slice()
        {
            // [(self . name) Type]
            [Symbol(x1), Symbol(x2), Symbol(name)]
                if x1 == keywords::SELF && x2 == operators::ACCESS =>
            {
                Some((name.clone(), MemberScope::Instance))
            }
            _ =>
            {
                // [(...) Type]
                context.add_error(
                    errors::FAILED_TYPE_DEFINITION,
                    "Unexpected list in type member name",
                    expression,
                );
                None
            }
        },
        _ =>
        {
            context.add_error(
                errors::FAILED_TYPE_DEFINITION,
                "Unexpected list in type member name",
                expression,
            );
            None
        }
    }
}

fn parse_data_block<'a>(
    elements: &'a [SExpression],
    context: &mut Context,
) -> (Vec<PartialMemberData<'a>>, Vec<ParseItemReference<'a>>)
{
    let mut members = Vec::new();
    let mut child_items = Vec::new();

    for element in elements
    {
        match element
        {
            List(BracketType::Square, member_elements) => match member_elements.as_slice()
            {
                [name, type_expression] => match parse_scoped_name(name, context)
                {
                    Some((name, scope)) =>
                    {
                        let member = (
                            name,
                            track_unparsed_child_item(
                                type_expression,
                                ParseItem::UnparsedType,
                                &mut child_items,
                            ),
                            scope,
                            Visibility::Private,
                            Visibility::Private,
                        );
                        members.push(member);
                    }
                    None =>
                    {
                        context.add_error(
                            errors::FAILED_TYPE_DEFINITION,
                            "Invalid type definition member name",
                            element,
                        );
                    }
                },
                _ =>
                {
                    context.add_error(
                        errors::FAILED_TYPE_DEFINITION,
                        "Invalid type definition member",
                        element,
                    );
                }
            },
            _ =>
            {
                context.add_error(
                    errors::FAILED_TYPE_DEFINITION,
                    "Invalid type definition member",
                    element,
                );
            }
        }
    }

    return (members, child_items);
}

fn parse_method_block<'a>(
    elements: &'a [SExpression],
    visibility: Visibility,
    context: &mut Context,
) -> (
    Vec<PartialMethodData<'a>>,
    Vec<(String, bool)>,
    Vec<(String, bool)>,
    Vec<ParseItemReference<'a>>,
)
{
    let mut methods = Vec::new();
    let mut public_read = Vec::new();
    let mut public_write = Vec::new();
    let mut child_items = Vec::new();

    for element in elements
    {
        match element
        {
            // (...)
            List(BracketType::Round, method_elements) =>
            {
                match method_elements.as_slice()
                {
                    // (read/read-write ...)
                    [Symbol(x), name] if x == keywords::READ || x == keywords::WRITE =>
                    {
                        match parse_scoped_name(name, context)
                        {
                            Some((name, scope)) =>
                            {
                                if visibility == Visibility::Public
                                {
                                    let is_instance_member = scope == MemberScope::Instance;
                                    let access = (name, is_instance_member);

                                    match x == keywords::WRITE
                                    {
                                        true => public_write.push(access),
                                        false => public_read.push(access),
                                    }
                                }
                                else
                                {
                                    context.add_warning(
                                        errors::FAILED_TYPE_DEFINITION,
                                        "Useless private access specifier in type definition",
                                        element,
                                    );
                                }
                            }

                            None =>
                            {
                                context.add_error(
                                    errors::FAILED_TYPE_DEFINITION,
                                    "Invalid type definition access name",
                                    element,
                                );
                            }
                        }
                    }

                    // (fn ...)
                    method_elements => match method_elements.first()
                    {
                        Some(Symbol(x))
                            if x == keywords::FUNCTION && method_elements.len() >= 3 =>
                        {
                            match parse_function::definition(method_elements, true, context)
                            {
                                Some((data, is_instance_method, mut new_child_items)) =>
                                {
                                    let scope = match is_instance_method
                                    {
                                        true => MemberScope::Instance,
                                        false => MemberScope::Static,
                                    };

                                    let method = (data, scope, visibility);
                                    methods.push(method);
                                    child_items.append(&mut new_child_items);
                                }
                                None =>
                                {
                                    context.add_error(
                                        errors::FAILED_TYPE_DEFINITION,
                                        "Failed to parse type definition method",
                                        element,
                                    );
                                }
                            }
                        }
                        _ =>
                        {
                            context.add_error(
                                errors::FAILED_TYPE_DEFINITION,
                                "Invalid type definition method",
                                element,
                            );
                        }
                    },
                }
            }

            _ =>
            {
                context.add_error(
                    errors::FAILED_TYPE_DEFINITION,
                    "Invalid type definition method",
                    element,
                );
            }
        }
    }

    return (methods, public_read, public_write, child_items);
}
