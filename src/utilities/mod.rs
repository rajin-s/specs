use std::cell::RefCell;
use std::rc::Rc;

pub use std::cell::{Ref, RefMut};

#[derive(Debug)]
pub struct Indirect<T>
{
    reference: Rc<RefCell<T>>,
}
impl<T> Indirect<T>
{
    pub fn new(value: T) -> Self
    {
        return Self {
            reference: Rc::new(RefCell::new(value)),
        };
    }
    pub fn clone(&self) -> Self
    {
        return Self {
            reference: Rc::clone(&self.reference),
        };
    }

    pub fn borrow(&self) -> Ref<T>
    {
        return Rc::as_ref(&self.reference).borrow();
    }
    pub fn borrow_mut(&self) -> RefMut<T>
    {
        return Rc::as_ref(&self.reference).borrow_mut();
    }

    pub fn unwrap(self) -> T
    {
        match Rc::try_unwrap(self.reference)
        {
            Ok(cell) => cell.into_inner(),
            Err(_) => panic!("Failed to unwrap Indirect"),
        }
    }
}
impl<T: std::fmt::Display> std::fmt::Display for Indirect<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.reference.as_ref().borrow())
    }
}
