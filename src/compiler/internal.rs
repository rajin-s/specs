use std::collections::VecDeque;

// All passes get access to basic stuff
pub use crate::language::*;
pub use crate::utilities::Indirect;
pub use node::*;

pub enum PassResult
{
    Ok(Vec<PassMessage>),
    Err(Vec<PassMessage>),
}

pub trait PassState
{
    fn empty() -> Self;
}

pub trait CompilerPass<TState: PassState>
{
    fn get_name(&self) -> String;
    fn transform(
        &self,
        node: &mut Node,
        state: Indirect<TState>,
        messages: &mut PassMessageContext,
    );
    fn get_state(
        &self,
        node: &Node,
        parent: Indirect<TState>,
        messages: &mut PassMessageContext,
    ) -> Indirect<TState>;
}

pub fn apply_compiler_pass<TState: PassState, TPass: CompilerPass<TState>>(
    pass: TPass,
    root_node: &mut OtherNode,
) -> PassResult
{
    // Create the queue for expanding/transforming nodes
    let mut queue = VecDeque::new();

    // Create a message context to use for the whole pass
    let mut message_context = PassMessageContext::new(pass.get_name());

    // Start with the root node on the queue
    let root_state = TState::empty();
    let root_item = (0, false, Indirect::new(root_state), root_node.clone());
    queue.push_front(root_item);

    // Continue expanding/transforming nodes until none are left
    loop
    {
        match queue.pop_front()
        {
            // The next node has not been expanded
            Some((depth, false, state, node)) =>
            {
                // Get all child nodes and shared child state
                let children = node.borrow().get_children();

                let child_state = {
                    let node_ref = &*node.borrow();
                    message_context.set_indent(depth);
                    pass.get_state(node_ref, state.clone(), &mut message_context)
                };

                // Put the parent item back on the queue, marking that it has been expanded
                let parent_item = (depth, true, state, node);
                queue.push_front(parent_item);

                // Put all child nodes on the queue BEFORE the parent
                //  note: added in reverse so they're processed in-order
                for child in children.into_iter().rev()
                {
                    let child_item = (depth + 1, false, child_state.clone(), child);
                    queue.push_front(child_item);
                }
            }

            // The next node has already been expanded, so it's ready to be transformed
            //  note: All child nodes would have been handled already, so this is the same as recursive order
            //  note: Children are processed in-order and may modify their shared state in an imperative fashion
            Some((depth, true, state, node)) =>
            {
                message_context.set_indent(depth);

                let node_ref = &mut *node.borrow_mut();
                pass.transform(node_ref, state, &mut message_context);
            }

            // No nodes left on the queue
            None =>
            {
                break;
            }
        }
    }

    let (messages, _has_warnings, has_errors) = message_context.destructure();
    if has_errors
    {
        PassResult::Err(messages)
    }
    else
    {
        PassResult::Ok(messages)
    }
}

/* -------------------------------------------------------------------------- */
/*                      Empty PassState for simple passes                     */
/* -------------------------------------------------------------------------- */

pub struct PassStateEmpty {}
impl PassStateEmpty
{
    pub fn get_state(
        _node: &Node,
        parent: Indirect<PassStateEmpty>,
        _messages: &mut PassMessageContext,
    ) -> Indirect<PassStateEmpty>
    {
        return parent.clone();
    }
}
impl PassState for PassStateEmpty
{
    fn empty() -> Self
    {
        return Self {};
    }
}

/* -------------------------------------------------------------------------- */
/*                            Pass Message Context                            */
/* -------------------------------------------------------------------------- */
pub struct PassMessage
{
    pass:    String,
    name:    String,
    message: String,
    source:  String,
    indent:  usize,
}
impl PassMessage
{
    pub fn new<TPass, TName, TMessage, TSource>(
        pass: TPass,
        name: TName,
        message: TMessage,
        source: TSource,
        indent: usize,
    ) -> Self
    where
        TPass: ToString,
        TName: ToString,
        TMessage: ToString,
        TSource: ToString,
    {
        return Self {
            pass: pass.to_string(),
            name: name.to_string(),
            message: message.to_string(),
            source: source.to_string(),
            indent,
        };
    }
}
impl std::fmt::Display for PassMessage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let name = format!("[{}::{}]", self.pass, self.name);
        let mut space: String = String::new();
        for _ in 0..name.len()
        {
            space.push(' ');
        }

        let mut indent = String::new();
        for _ in 0..self.indent
        {
            indent.push('\t');
        }

        write!(
            f,
            "{}{}\n{}- '{}'\n{}\t@ {}",
            &indent, name, &indent, self.message, &indent, self.source,
        )
    }
}

pub struct PassMessageContext
{
    messages: Vec<PassMessage>,

    pass_name:      String,
    current_indent: usize,

    pub has_errors:   bool,
    pub has_warnings: bool,
}
impl PassMessageContext
{
    pub fn new(pass_name: String) -> Self
    {
        return Self {
            messages: Vec::new(),

            pass_name,
            current_indent: 0,

            has_errors: false,
            has_warnings: false,
        };
    }

    pub fn add_error<TName: ToString, TMessage: ToString, TSource: ToString>(
        &mut self,
        name: TName,
        message: TMessage,
        source: TSource,
    )
    {
        let pass = format!("ERROR: {}", self.pass_name);
        self.messages.push(PassMessage::new(
            pass,
            name,
            message,
            source,
            self.current_indent,
        ));
        self.has_errors = true;
    }
    pub fn add_warning<TName: ToString, TMessage: ToString, TSource: ToString>(
        &mut self,
        name: TName,
        message: TMessage,
        source: TSource,
    )
    {
        let pass = format!("warning: {}", self.pass_name);
        self.messages.push(PassMessage::new(
            pass,
            name,
            message,
            source,
            self.current_indent,
        ));
        self.has_warnings = true;
    }
    pub fn set_indent(&mut self, indent: usize)
    {
        self.current_indent = indent;
    }

    pub fn destructure(self) -> (Vec<PassMessage>, bool, bool)
    {
        return (self.messages, self.has_warnings, self.has_errors);
    }
}
