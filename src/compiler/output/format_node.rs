use super::*;
use crate::language::symbols;

pub fn get_functions(node: &Node, declarations_only: bool) -> String
{
    let mut result = String::new();
    let mut context = FormatContext::new(&mut result);

    context.declaration_mode = declarations_only;
    parse_function_definitions(node, &mut context);

    return result;
}
pub fn get_types(node: &Node, declarations_only: bool) -> String
{
    let mut result = String::new();
    let mut context = FormatContext::new(&mut result);

    context.declaration_mode = declarations_only;
    parse_type_definitions(node, &mut context);

    return result;
}
pub fn get_program_body(node: &Node) -> String
{
    let mut result = String::new();
    let mut context = FormatContext::new(&mut result);
    parse_node(node, &mut context);
    return result;
}

fn parse_node(node: &Node, context: &mut FormatContext)
{
    match node
    {
        Node::Nothing =>
        {
            context.write_comment_str("invalid nothing");
        }
        Node::Integer(data) =>
        {
            let s = format!("{}", data.get_value());
            context.write(s);
        }
        Node::Boolean(data) =>
        {
            let s = format!("{}", data.get_value());
            context.write(s);
        }
        Node::Variable(data) =>
        {
            let s = context.get_identifier(data.get_name());
            context.write(s);
        }

        Node::Call(data) =>
        {
            match data.get_operator()
            {
                Node::PrimitiveOperator(operator_data) =>
                {
                    match data.get_operands().len()
                    {
                        1 =>
                        {
                            let operator_string = match operator_data.get_operator()
                            {
                                _ => "__OPERATOR__",
                            };
                            if context.get_last_group() == "condition"
                            {
                                context.write(format!("{} ", operator_string));
                                parse_node(&data.get_operands()[0], context);
                            }
                            else
                            {
                                context.push_group(operator_string, true, false);
                                {
                                    context.write(format!("{} ", operator_string));
                                    parse_node(&data.get_operands()[0], context);
                                }
                                context.pop_group();
                            }
                        }
                        2 =>
                        {
                            let operator_string = match operator_data.get_operator()
                            {
                                // Arithmetic operators
                                PrimitiveOperator::Add => "+",

                                // Comparison operators
                                PrimitiveOperator::Equal => "==",
                                PrimitiveOperator::NotEqual => "!=",
                                PrimitiveOperator::Less => "<",
                                PrimitiveOperator::Greater => ">",
                                PrimitiveOperator::LessEqual => "<=",
                                PrimitiveOperator::GreaterEqual => ">=",

                                // Logical operators
                                PrimitiveOperator::And => "&&",
                                PrimitiveOperator::Or => "||",
                                PrimitiveOperator::ExclusiveOr => "^",

                                _ => "_OPERATOR_",
                            };

                            if context.get_last_group() == "condition"
                            {
                                parse_node(&data.get_operands()[0], context);
                                context.write(format!(" {} ", operator_string));
                                parse_node(&data.get_operands()[1], context);
                            }
                            else
                            {
                                context.push_group(operator_string, true, false);
                                {
                                    parse_node(&data.get_operands()[0], context);
                                    context.write(format!(" {} ", operator_string));
                                    parse_node(&data.get_operands()[1], context);
                                }
                                context.pop_group();
                            }
                        }
                        _ =>
                        {
                            context.write_comment_str("Invalid call primitive operarand");
                        }
                    }
                }
                Node::Variable(variable_data) if variable_data.get_name() == "return" =>
                {
                    context.push_group("return", false, false);
                    {
                        context.write_str("return ");
                        parse_node(&data.get_operands()[0], context);
                    }
                    context.pop_group();
                }
                _ =>
                {
                    parse_node(data.get_operator(), context);
                    context.push_group("function_args", true, true);

                    for (i, operand) in data.get_operands().iter().enumerate()
                    {
                        if i > 0
                        {
                            context.write_str(", ");
                        }

                        parse_node(operand, context);
                    }

                    context.pop_group();
                }
            }
        }
        Node::PrimitiveOperator(data) =>
        {
            let s = format!("invalid primitive {:?}", data.get_operator());
            context.write_comment(s);
        }

        Node::Reference(data) =>
        {
            context.push_group("ref", true, false);
            {
                context.write_str("&");
                parse_node(data.get_target(), context);
            }
            context.pop_group();
        }
        Node::Dereference(data) =>
        {
            context.push_group("deref", true, false);
            {
                context.write_str("*");
                parse_node(data.get_target(), context);
            }
            context.pop_group();
        }

        Node::Binding(data) =>
        {
            let s = context.get_typed_identifier(data.get_name(), data.get_binding_type());
            context.write(s);

            match data.get_binding()
            {
                Node::Nothing =>
                {}
                _ =>
                {
                    context.write_str(" = ");
                    context.push_group("binding", false, false);
                    {
                        parse_node(data.get_binding(), context);
                    }
                    context.pop_group();
                }
            }
        }
        Node::Assignment(data) =>
        {
            context.push_group("assign", false, false);
            {
                parse_node(data.get_lhs(), context);
                context.write_str(" = ");
                parse_node(data.get_rhs(), context);
            }
            context.pop_group();
        }

        Node::Sequence(data) =>
        {
            context.push_group("sequence", false, false);
            {
                if !data.is_transparent()
                {
                    context.write_str("{");
                }
                context.indent();
                for node in data.get_nodes().iter()
                {
                    context.start_line();
                    parse_node(node, context);
                    context.end_line(node);
                }
                context.dedent();
                context.start_line();

                if !data.is_transparent()
                {
                    context.write_str("}");
                }
            }
            context.pop_group();
        }
        Node::Conditional(data) =>
        {
            context.push_group("conditional", false, false);
            {
                context.write_str("if ");
                context.push_group("condition", true, true);
                {
                    parse_node(data.get_condition(), context);
                }
                context.pop_group();

                let then_needs_bracket = match data.get_then()
                {
                    Node::Sequence(sequence_data) => sequence_data.is_transparent(),
                    _ => true,
                };
                context.start_line();
                if then_needs_bracket
                {
                    context.write_str("{");
                    context.indent();
                    context.start_line();

                    parse_node(data.get_then(), context);
                    context.end_line(data.get_then());

                    context.dedent();
                    context.start_line();
                    context.write_str("}");
                }
                else
                {
                    parse_node(data.get_then(), context);
                }

                if data.has_else()
                {
                    context.start_line();
                    context.write_str("else");
                    let else_needs_bracket = match data.get_else()
                    {
                        Node::Sequence(sequence_data) => sequence_data.is_transparent(),
                        _ => true,
                    };
                    context.start_line();
                    if else_needs_bracket
                    {
                        context.write_str("{");
                        context.indent();
                        context.start_line();

                        parse_node(data.get_else(), context);
                        context.end_line(data.get_else());

                        context.dedent();
                        context.start_line();
                        context.write_str("}");
                    }
                    else
                    {
                        parse_node(data.get_else(), context);
                    }
                }
            }
            context.pop_group();
        }

        Node::Function(data) =>
        {
            let s = format!("function {}", data.get_name());
            context.write_comment(s);
        }

        Node::Type(data) =>
        {
            let s = format!("type {}", data.get_name());
            context.write_comment(s);
        }
        Node::Access(data) =>
        {
            // See if we're accessing a static member of a type
            let target_type_name = match data.get_target()
            {
                Node::Variable(variable_data) => match variable_data.get_type().get_data_type()
                {
                    DataType::Type(_) => Some(variable_data.get_name()),
                    _ => None,
                },
                _ => None,
            };

            // Handle static method and data access
            if let Some(name) = target_type_name
            {
                let is_static_method = if let DataType::Function(function_data) =
                    data.get_type().get_data_type()
                {
                    let function_type = function_data.get_metadata().get_type();
                    if function_type == FunctionType::InstanceMethod
                        || function_type == FunctionType::StaticMethod
                    {
                        true
                    }
                    else
                    {
                        false
                    }
                }
                else
                {
                    false
                };

                if is_static_method
                {
                    context.write(format!("{}__{}", name, data.get_property()));
                }
                else
                {
                    context.push_group("access", true, false);
                    {
                        context.write(format!("{}__static", name));
                        context.write_str(".");
                        context.write(data.get_property().clone());
                    }
                    context.pop_group();
                }
            }
            else
            {
                // Target is not a type

                context.push_group("access", true, false);
                {
                    parse_node(data.get_target(), context);
                    let target_type = data.get_target().get_type();
                    let is_pointer = target_type.is_reference()
                        /* || !target_type.has_trait_str(symbols::traits::VALUE) */;

                    if is_pointer
                    {
                        context.write_str("->");
                    }
                    else
                    {
                        context.write_str(".");
                    }
                    context.write(data.get_property().clone());
                }
                context.pop_group();
            }
        }
    }
}

fn parse_function_definitions(node: &Node, context: &mut FormatContext)
{
    fn write_function(
        data: &FunctionNodeData,
        type_name: Option<&String>,
        context: &mut FormatContext,
    )
    {
        let return_type_string = context.get_type(data.get_return_type());

        let function_name = match type_name
        {
            Some(name) => format!("{}__{}", name, context.get_identifier(data.get_name())),
            None => context.get_identifier(data.get_name()),
        };

        context.start_line();
        context.write(format!("{} {}", return_type_string, function_name));

        context.push_group("function_args", true, true);
        for (i, argument) in data.get_arguments().iter().enumerate()
        {
            let argument_string =
                context.get_typed_identifier(argument.get_name(), argument.get_type());

            if i > 0
            {
                context.write_str(", ");
            }

            context.write(argument_string);
        }
        context.pop_group();

        if context.declaration_mode
        {
            context.end_line(&Node::Nothing);
        }
        else
        {
            context.start_line();
            let body_needs_bracket = match data.get_body()
            {
                Node::Sequence(_) => false,
                _ => true,
            };
            if body_needs_bracket
            {
                context.write_str("{");
                context.indent();
                context.start_line();
                parse_node(data.get_body(), context);
                context.end_line(data.get_body());
                context.dedent();
                context.start_line();
                context.write_str("}");
            }
            else
            {
                parse_node(data.get_body(), context);
            }
        }
    }

    match node
    {
        Node::Function(data) =>
        {
            write_function(data, None, context);
            node.recur_parse(parse_function_definitions, context);
        }
        Node::Type(data) =>
        {
            for method in data.get_methods().iter()
            {
                let type_name = context.get_identifier(data.get_name());
                write_function(method.get_function_data(), Some(&type_name), context);
            }
            node.recur_parse(parse_function_definitions, context);
        }
        _ =>
        {
            node.recur_parse(parse_function_definitions, context);
        }
    }
}
fn parse_type_definitions(node: &Node, context: &mut FormatContext)
{
    fn write_struct(data: &TypeNodeData, context: &mut FormatContext)
    {
        let type_name = context.get_identifier(data.get_name());

        let mut has_static_members = false;
        for member in data.get_members().iter()
        {
            if member.get_scope() == MemberScope::Static
            {
                has_static_members = true;
                break;
            }
        }

        if context.declaration_mode
        {
            context.start_line();
            context.write(format!("struct {}", type_name));
            context.end_line(&Node::Nothing);
        }
        else
        {
            // Instance members
            {
                context.start_line();
                context.write(format!("typedef struct {}", type_name));

                context.start_line();
                context.write_str("{");
                context.indent();

                for member in data.get_members().iter()
                {
                    if member.get_scope() == MemberScope::Instance
                    {
                        let member_string =
                            context.get_typed_identifier(member.get_name(), member.get_type());
                        context.start_line();
                        context.write(format!("{}", member_string));
                        context.end_line(&Node::Nothing);
                    }
                }

                context.dedent();
                context.start_line();
                context.write_str("}");

                context.write(format!(" {}", type_name));
                context.end_line(&Node::Nothing);
                context.start_line();
            }

            // Static members
            if has_static_members
            {
                context.start_line();
                context.write_str("struct");

                context.start_line();
                context.write_str("{");
                context.indent();

                for member in data.get_members().iter()
                {
                    if member.get_scope() == MemberScope::Static
                    {
                        let member_string =
                            context.get_typed_identifier(member.get_name(), member.get_type());
                        context.start_line();
                        context.write(format!("{};", member_string));
                    }
                }

                context.dedent();
                context.start_line();
                context.write_str("}");

                context.write(format!(" {}__static;", type_name));
            }
        }
    }

    match node
    {
        Node::Type(data) =>
        {
            write_struct(data, context);
            node.recur_parse(parse_type_definitions, context);
        }
        _ =>
        {
            node.recur_parse(parse_type_definitions, context);
        }
    }
}

struct FormatContext<'a>
{
    result:       &'a mut String,
    group_names:  Vec<(String, bool)>,
    indent_level: usize,

    pub declaration_mode: bool,
}
impl<'a> FormatContext<'a>
{
    pub fn write(&mut self, content: String)
    {
        *self.result = format!("{}{}", self.result, content);
    }
    pub fn write_str(&mut self, content: &str)
    {
        self.write(content.to_owned());
    }
    pub fn write_comment(&mut self, content: String)
    {
        *self.result = format!("{}/* {} */", self.result, content);
    }
    pub fn write_comment_str(&mut self, content: &str)
    {
        *self.result = format!("{}/* {} */", self.result, content);
    }

    pub fn get_identifier(&self, name: &String) -> String
    {
        return symbols::convert_to_c_safe(name);
    }
    pub fn get_type(&self, t: &Type) -> String
    {
        let type_string = match t.get_data_type()
        {
            DataType::Integer => "int",
            DataType::Boolean => "bool",
            DataType::Void => "void",
            DataType::Unknown => "_UNKNOWN_",
            DataType::Function(_) => "_CALLABLE_",
            DataType::Type(_) => "_TYPE_",
            DataType::Instance(data) => data.get_name().as_str(),
        };

        let mut reference_string = String::new();
        for (i, _) in t.get_reference_layers().iter().enumerate()
        {
            if i == 0
            {
                reference_string.push(' ');
            }
            reference_string.push('*');
        }

        // By default, structs are passed by reference
        if reference_string.is_empty() // && !t.has_trait_str(symbols::traits::VALUE)
        {
            reference_string = String::from("*");
        }

        return format!("{}{}", type_string, reference_string);
    }
    pub fn get_typed_identifier(&self, name: &String, t: &Type) -> String
    {
        match t.get_data_type()
        {
            DataType::Function(_data) => format!("_FUNCTION_ {}", self.get_identifier(name)),
            _ => format!("{} {}", self.get_type(t), self.get_identifier(name)),
        }
    }

    pub fn push_group(&mut self, name: &str, mut use_paren: bool, force_paren: bool)
    {
        match self.group_names.last()
        {
            Some((group_name, _)) if group_name == name =>
            {
                use_paren = force_paren;
            }
            _ =>
            {}
        }

        self.group_names.push((name.to_owned(), use_paren));
        if use_paren
        {
            *self.result = format!("{}(", self.result);
        }
    }
    pub fn pop_group(&mut self)
    {
        let group = self.group_names.pop();
        if let Some((_, true)) = group
        {
            *self.result = format!("{})", self.result);
        }
    }
    pub fn get_last_group(&self) -> String
    {
        match self.group_names.last()
        {
            Some((name, _)) => name.clone(),
            None => String::new(),
        }
    }

    pub fn start_line(&mut self)
    {
        let mut indent_string = String::new();
        for _ in 0..self.indent_level
        {
            indent_string.push('\t');
        }

        *self.result = format!("{}\n{}", self.result, indent_string);
    }
    pub fn end_line(&mut self, node: &Node)
    {
        match node
        {
            Node::Sequence(_) | Node::Conditional(_) | Node::Function(_) =>
            {}
            _ =>
            {
                self.result.push(';');
            }
        }
    }

    pub fn indent(&mut self)
    {
        self.indent_level += 1;
    }
    pub fn dedent(&mut self)
    {
        self.indent_level -= 1;
    }

    pub fn new(result: &'a mut String) -> Self
    {
        return Self {
            result:           result,
            group_names:      Vec::new(),
            indent_level:     0,
            declaration_mode: false,
        };
    }
}
