use super::internal::*;
use imports::*;

pub fn definition<'a>(
    elements: &'a [SExpression],
    allow_instance_methods: bool,
    context: &mut Context,
) -> Option<(PartialFunctionData<'a>, bool, Vec<ParseItemReference<'a>>)>
{
    // Track all newly created child items
    let mut child_items = Vec::new();

    // Helper to create an Rc to an unparsed expression item and add a copy to the list of child items
    let mut child = |expression, f: fn(&'a SExpression) -> ParseItem<'a>| {
        let new_ref = Rc::new(f(expression));
        child_items.push(new_ref.clone());
        new_ref
    };

    // Get the function name and check if it's an instance method
    let (name, is_instance_method) = match &elements[1]
    {
        // Symbol name (basic function / static method)
        Symbol(name) => (name.clone(), false),

        // List name (self . name) if instance methods are allowed
        List(BracketType::Round, name_elements) if allow_instance_methods => match name_elements
            .as_slice()
        {
            // (self . name)
            [Symbol(x1), Symbol(x2), Symbol(name)]
                if x1 == keywords::SELF && x2 == operators::ACCESS =>
            {
                (name.clone(), true)
            }
            _ =>
            {
                context.add_error(
                    errors::FAILED_FUNCTION,
                    "Invalid list in function name",
                    &elements[1],
                );
                return None;
            }
        },
        _ =>
        {
            context.add_error(
                errors::FAILED_FUNCTION,
                "Unexpected list in function name",
                &elements[1],
            );
            return None;
        }
    };

    // Get the function arguments, return type, and body expressions
    let (arguments, return_type, body) = match &elements[2..]
    {
        // fn name {...}
        [List(BracketType::Curly, _)] =>
        {
            (None, None, &elements[2])
        }
        // fn name <...> {...}
        [List(BracketType::None, argument_elements), List(BracketType::Curly, _)] =>
        {
            (Some(argument_elements), None, &elements[3])
        }
        // fn name -> Type {...}
        [Symbol(x1), return_type, List(BracketType::Curly, _)] if x1 == keywords::RETURNS =>
        {
            (None, Some(return_type), &elements[4])
        }
        // fn name <...> -> Type {...}
        [List(BracketType::None, argument_elements), Symbol(x1), return_type, List(BracketType::Curly, _)]
            if x1 == keywords::RETURNS =>
        {
            (Some(argument_elements), Some(return_type), &elements[5])
        }
        _ =>
        {
            context.add_error(
                errors::FAILED_FUNCTION,
                "Invalid function arguments / return type layout",
                List(BracketType::Round, elements.to_vec()),
            );
            return None;
        }
    };

    // If a return type was given, create a new child ParseItem, otherwise use a complete void type
    //  note: void type isn't added to child_items, since it doesn't need to onto the parse queue
    let return_type = match return_type
    {
        Some(expression) => child(expression, ParseItem::UnparsedType),
        None => Rc::new(ParseItem::CompleteType(basic_types::void())),
    };

    let arguments = match arguments
    {
        Some(arguments) =>
        {
            let argument_count = arguments.len();

            // Create child ParseItems for each argument type
            let arguments: Vec<_> = arguments
                .iter()
                .filter_map(|argument| match parse_argument(argument, context)
                {
                    Some((name, type_expression)) =>
                    {
                        Some((name, child(type_expression, ParseItem::UnparsedType)))
                    }
                    None => None,
                })
                .collect();

            // Make sure all arguments were properly parsed
            if argument_count == arguments.len()
            {
                arguments
            }
            else
            {
                context.add_error(
                    errors::FAILED_FUNCTION,
                    "Failed to parse all function arguments",
                    List(BracketType::Round, elements.to_vec()),
                );
                return None;
            }
        }
        None => Vec::new(),
    };

    // Create child ParseItem for the function body
    let body = child(body, ParseItem::UnparsedNode);

    let function_data = (name, arguments, return_type, body);
    Some((function_data, is_instance_method, child_items))
}

fn parse_argument<'a>(
    expression: &'a SExpression,
    context: &mut Context,
) -> Option<(String, &'a SExpression)>
{
    match expression
    {
        List(BracketType::Square, elements) => match elements.as_slice()
        {
            [Symbol(name), type_expression] => Some((name.clone(), type_expression)),
            _ =>
            {
                context.add_error(
                    errors::FAILED_ARGUMENT,
                    "Invalid [...] list in function arguments",
                    expression,
                );
                None
            }
        },

        _ =>
        {
            context.add_error(
                errors::FAILED_ARGUMENT,
                "Unexpected list in function arguments",
                expression,
            );
            None
        }
    }
}