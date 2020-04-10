use crate::lang_rml::nodes::*;
use crate::lang_rml::symbols;

use crate::lang_sexp::sexp::*;
use SExpression::*;

use super::common::*;
use super::parse_node;

/* -------------------------------------------------------------------------- */
/*                                  Function                                  */
/* -------------------------------------------------------------------------- */

pub fn parse_function(
    pattern_expressions: &Vec<SExpression>,
    body_expressions: &Vec<SExpression>,
    return_type: FullType,
) -> Option<Node>
{
    if let Some(Symbol(name)) = pattern_expressions.first()
    {
        let mut arguments: Vec<ArgumentData> = Vec::new();
        for expression in &pattern_expressions[1..]
        {
            if let Some(argument) = parse_argument(expression)
            {
                arguments.push(argument);
            }
            else
            {
                parse_error(
                    "Failed to parse function pattern (bad argument)",
                    &List(BracketType::Round, pattern_expressions.clone()),
                );
                return None;
            }
        }

        let mut body_nodes: Vec<TypedNode> = Vec::new();
        for expression in body_expressions
        {
            if let Some(node) = parse_node(expression)
            {
                body_nodes.push(node.to_unknown_typed());
            }
            else
            {
                parse_error("Failed to parse function body expression", expression);
                return None;
            }
        }

        return Some(FunctionData::new(name.clone(), return_type, arguments, body_nodes).to_node());
    }
    else
    {
        parse_error(
            "Failed to parse function pattern (no name)",
            &List(BracketType::Round, pattern_expressions.clone()),
        );
        return None;
    }

    // Helper for arguments
    fn parse_argument(expression: &SExpression) -> Option<ArgumentData>
    {
        match expression
        {
            Symbol(name) => Some(ArgumentData::new(name.clone(), FullType::unknown())),
            List(_, elements) => match elements.as_slice()
            {
                [Symbol(name), type_expression] =>
                {
                    let argument_type = FullType::from_sexp(type_expression);
                    return Some(ArgumentData::new(name.clone(), argument_type));
                }
                _ =>
                {
                    parse_error("Failed to parse function argument", expression);
                    return None;
                }
            },
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Struct                                   */
/* -------------------------------------------------------------------------- */

pub fn parse_struct(name: &String, content_groups: &Vec<SExpression>) -> Option<Node>
{
    use symbols::keywords::*;

    // accumulated struct members and methods
    let mut members: Vec<MemberData> = Vec::new();
    let mut methods: Vec<MethodData> = Vec::new();

    for expression in content_groups.iter()
    {
        if let List(_, group_elements) = expression
        {
            match group_elements.as_slice()
            {
                [Symbol(public), List(_, content_expressions)] if public == PUBLIC =>
                {
                    // (public ( content... ))
                    if let Some((mut group_members, mut group_methods)) =
                        parse_struct_content(content_expressions, Visibility::Public)
                    {
                        members.append(&mut group_members);
                        methods.append(&mut group_methods);
                    }
                    else
                    {
                        parse_error("Failed to parse public struct content", expression);
                        return None;
                    }
                }

                [Symbol(private), List(_, content_expressions)] if private == PRIVATE =>
                {
                    // (private ( content... ))
                    if let Some((mut group_members, mut group_methods)) =
                        parse_struct_content(content_expressions, Visibility::Private)
                    {
                        members.append(&mut group_members);
                        methods.append(&mut group_methods);
                    }
                    else
                    {
                        parse_error("Failed to parse private struct content", expression);
                        return None;
                    }
                }
                _ =>
                {
                    parse_error(
                        "Failed to parse struct (invalid content layout)",
                        expression,
                    );
                    return None;
                }
            }
        }
        else
        {
            parse_error(
                "Failed to parse struct (invalid content layout)",
                expression,
            );
            return None;
        }
    }

    return Some(StructData::new(name.clone(), members, methods).to_node());

    // Helper for content groups
    fn parse_struct_content(
        content_expressions: &Vec<SExpression>,
        visibility: Visibility,
    ) -> Option<(Vec<MemberData>, Vec<MethodData>)>
    {
        let mut members: Vec<MemberData> = Vec::new();
        let mut methods: Vec<MethodData> = Vec::new();
        for expression in content_expressions.iter()
        {
            if let List(_, expression_elements) = expression
            {
                if let [Symbol(member_name), member_type] = expression_elements.as_slice()
                {
                    members.push(MemberData::new(
                        member_name.clone(),
                        FullType::from_sexp(member_type),
                        visibility,
                    ));
                }
                else if let Some(Node::Function(function_data)) = parse_node(expression)
                {
                    methods.push(MethodData::new(function_data, visibility));
                }
                else
                {
                    parse_error(
                        "Failed to parse struct content (invalid layout)",
                        expression,
                    );
                    return None;
                }
            }
            else
            {
                parse_error(
                    "Failed to parse struct content (invalid layout)",
                    expression,
                );
                return None;
            }
        }
        return Some((members, methods));
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Binding                                  */
/* -------------------------------------------------------------------------- */

pub fn parse_binding(name: &String, binding_expression: &SExpression) -> Option<Node>
{
    if let Some(binding) = parse_node(binding_expression)
    {
        return Some(BindingData::new(name.clone(), binding.to_unknown_typed()).to_node());
    }
    else
    {
        parse_error(
            "Failed to parse binding (invalid expression)",
            binding_expression,
        );
        return None;
    }
}

/* -------------------------------------------------------------------------- */
/*                                  Operator                                  */
/* -------------------------------------------------------------------------- */

pub fn parse_operator(
    operator_expression: &SExpression,
    operand_expressions: &Vec<SExpression>,
) -> Option<Node>
{
    use super::parse_primitive::parse_primitive_operator;

    let mut operands: Vec<TypedNode> = Vec::new();
    for expression in operand_expressions.iter()
    {
        if let Some(operand) = parse_node(expression)
        {
            operands.push(operand.to_unknown_typed());
        }
        else
        {
            parse_error("Failed to parse operand", expression);
            return None;
        }
    }

    if let Some(operator) = parse_primitive_operator(operator_expression, operand_expressions.len())
    {
        return Some(OperatorData::new(operator.to_unknown_typed(), operands).to_node());
    }
    else if let Some(operator) = parse_node(operator_expression)
    {
        return Some(OperatorData::new(operator.to_unknown_typed(), operands).to_node());
    }
    else
    {
        parse_error("Failed to parse operator", operator_expression);
        return None;
    }
}
pub fn parse_operator_ref(
    operator_expression: &SExpression,
    operand_expressions: &Vec<&SExpression>,
) -> Option<Node>
{
    use super::parse_primitive::parse_primitive_operator;

    let mut operands: Vec<TypedNode> = Vec::new();
    for expression in operand_expressions.iter()
    {
        if let Some(operand) = parse_node(expression)
        {
            operands.push(operand.to_unknown_typed());
        }
        else
        {
            parse_error("Failed to parse operand", expression);
            return None;
        }
    }

    if let Some(operator) = parse_primitive_operator(operator_expression, operand_expressions.len())
    {
        return Some(OperatorData::new(operator.to_unknown_typed(), operands).to_node());
    }
    else if let Some(operator) = parse_node(operator_expression)
    {
        return Some(OperatorData::new(operator.to_unknown_typed(), operands).to_node());
    }
    else
    {
        parse_error("Failed to parse operator", operator_expression);
        return None;
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Conditional                                */
/* -------------------------------------------------------------------------- */

pub fn parse_if(
    condition_expression: &SExpression,
    then_expressions: &Vec<SExpression>,
    else_expressions: &Vec<SExpression>,
) -> Option<Node>
{
    if let Some(condition_node) = parse_node(condition_expression)
    {
        let mut then_nodes: Vec<TypedNode> = Vec::new();
        let mut else_nodes: Vec<TypedNode> = Vec::new();

        for expression in then_expressions.iter()
        {
            if let Some(then_node) = parse_node(expression)
            {
                then_nodes.push(then_node.to_unknown_typed());
            }
            else
            {
                parse_error("Failed to parse if then-branch expression", expression);
                return None;
            }
        }
        for expression in else_expressions.iter()
        {
            if let Some(else_node) = parse_node(expression)
            {
                else_nodes.push(else_node.to_unknown_typed());
            }
            else
            {
                parse_error("Failed to parse if else-branch expression", expression);
                return None;
            }
        }

        return Some(
            IfData::new(condition_node.to_unknown_typed(), then_nodes, else_nodes).to_node(),
        );
    }
    else
    {
        parse_error("Failed to parse if conditional", condition_expression);
        return None;
    }
}
