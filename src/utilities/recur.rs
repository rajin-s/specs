use super::*;
use crate::errors::*;

use std::collections::VecDeque;

///
/// Trait for recursive structures
///
pub trait Recur<TItem>: Sized
{
    fn get_children(&self) -> Vec<&TItem>
    {
        Vec::new()
    }
    fn get_children_mut(&mut self) -> Vec<&mut TItem>
    {
        Vec::new()
    }
}

pub enum ChildState<T>
{
    Inherit,
    New(T),
}

///
/// Recursive transformation over some recursive structure with some state
///
pub trait RecurTransform<TItem, TState, TError>: Sized
where
    TItem: Recur<TItem>,
    TState: Sized,
    TError: ErrorTrait,
{
    fn get_root_state(&mut self, root: &TItem) -> TState;
    fn get_child_states(&mut self, state: &TState, item: &TItem) -> Vec<ChildState<TState>>
    {
        vec![ChildState::Inherit]
    }

    ///
    /// Apply the transformation to a root item using a RecurQueueMut
    ///
    fn apply(self, root: &mut TItem) -> ResultLog<(), TError>
    {
        let queue = RecurQueueMut::new(root, self);
        queue.apply()
    }

    ///
    /// Potentially modify an item BEFORE its children are processed (or added to the queue at all)
    ///
    fn enter(&mut self, _item: &mut TItem, _state: &mut TState) -> ResultLog<(), TError>
    {
        ResultLog::Ok(())
    }

    ///
    /// Potentially modify an item AFTER its children are processed
    ///
    fn exit(&mut self, _item: &mut TItem, _state: &mut TState) -> ResultLog<(), TError>
    {
        ResultLog::Ok(())
    }
}

///
/// Queue for performing a recursive transformation without blowing up the stack
///
struct RecurQueueMut<TItem, TState, TError, TTransform>
where
    TItem: Recur<TItem>,
    TState: Sized,
    TError: ErrorTrait,
    TTransform: RecurTransform<TItem, TState, TError>,
{
    queue:     VecDeque<(*mut TItem, Indirect<TState>, bool)>,
    transform: TTransform,

    _error: std::marker::PhantomData<TError>,
}

impl<TItem, TState, TError, TTransform> RecurQueueMut<TItem, TState, TError, TTransform>
where
    TItem: Recur<TItem>,
    TState: Sized,
    TError: ErrorTrait,
    TTransform: RecurTransform<TItem, TState, TError>,
{
    pub fn new(root: &mut TItem, mut transform: TTransform) -> Self
    {
        let root_ptr = root as *mut TItem;
        let root_state = transform.get_root_state(root);

        let mut queue = VecDeque::new();
        queue.push_front((root_ptr, Indirect::new(root_state), false));

        Self {
            queue,
            transform,
            _error: std::marker::PhantomData,
        }
    }

    pub fn apply(mut self) -> ResultLog<(), TError>
    where
        TTransform: RecurTransform<TItem, TState, TError>,
    {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        loop
        {
            // Get the next item of the queue, or exit if we're done
            let (next_ptr, next_state, next_is_entered) = match self.queue.pop_front()
            {
                Some((next_ptr, next_state, next_is_entered)) =>
                {
                    (next_ptr, next_state, next_is_entered)
                }
                None => break,
            };

            let next = unsafe { &mut *next_ptr };

            if next_is_entered
            {
                // The next item has already been expanded, so we can go ahead and do the transformation
                let mut state = next_state.borrow_mut();
                match self.transform.exit(next, &mut state)
                {
                    ResultLog::Ok(()) => (),
                    ResultLog::Warn((), mut new_warnings) => warnings.append(&mut new_warnings),
                    ResultLog::Error(mut new_errors, mut new_warnings) =>
                    {
                        errors.append(&mut new_errors);
                        warnings.append(&mut new_warnings)
                    }
                }
            }
            else
            {
                // The next item has not been expanded, so we add it back onto the queue in an
                //  expanded state, then add all child items in front so we come back to the
                //  original item AFTER its children have been handled

                match self.transform.enter(next, &mut next_state.borrow_mut())
                {
                    ResultLog::Ok(()) => (),
                    ResultLog::Warn((), mut new_warnings) => warnings.append(&mut new_warnings),
                    ResultLog::Error(mut new_errors, mut new_warnings) =>
                    {
                        errors.append(&mut new_errors);
                        warnings.append(&mut new_warnings)
                    }
                }

                // Get child item states

                let parent_state = next_state.clone();
                let child_states = self
                    .transform
                    .get_child_states(&parent_state.borrow(), next);

                let children = next.get_children_mut();

                // Add the original item back onto the queue

                self.queue.push_front((next_ptr, next_state, true));

                // Add child items / states onto the queue
                //  NOTE: Children are added in reverse order, so they're processed in the order
                //          they actually appear

                if child_states.len() == 1
                {
                    // All children share the same state

                    let state = match child_states.into_1()
                    {
                        ChildState::Inherit => parent_state.clone(),
                        ChildState::New(state) => Indirect::new(state),
                    };

                    for child in children.into_iter().rev()
                    {
                        let child_ptr = child as *mut TItem;
                        self.queue.push_front((child_ptr, state.clone(), false));
                    }
                }
                else if child_states.len() == children.len()
                {
                    // Each child gets its own state

                    for (child, child_state) in
                        children.into_iter().zip(child_states.into_iter()).rev()
                    {
                        let state = match child_state
                        {
                            ChildState::Inherit => parent_state.clone(),
                            ChildState::New(state) => Indirect::new(state),
                        };

                        let child_ptr = child as *mut TItem;
                        self.queue.push_front((child_ptr, state, false));
                    }
                }
                else
                {
                    panic!(
                        "RecurQueueMut: Expected {} child states, got {}",
                        children.len(),
                        child_states.len()
                    );
                }
            }
        }

        ResultLog::maybe_error((), warnings, errors)
    }
}