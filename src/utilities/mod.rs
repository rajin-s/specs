mod recur;

use std::cell::RefCell;
use std::rc::Rc;

pub use std::cell::{Ref, RefMut};
pub use recur::*;

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
impl<T> std::clone::Clone for Indirect<T>
{
    fn clone(&self) -> Self
    {
        return Self {
            reference: Rc::clone(&self.reference),
        };
    }
}
impl<T: std::fmt::Display> std::fmt::Display for Indirect<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.reference.as_ref().borrow())
    }
}

pub trait IntoN<T>
{
    fn into_1(self) -> T;
    fn into_2(self) -> (T, T);
    fn into_3(self) -> (T, T, T);
    fn into_4(self) -> (T, T, T, T);
    
    fn try_into_1(self) -> Option<T>;
    fn try_into_2(self) -> (Option<T>, Option<T>);
    fn try_into_3(self) -> (Option<T>, Option<T>, Option<T>);
    fn try_into_4(self) -> (Option<T>, Option<T>, Option<T>, Option<T>);
}
impl<T> IntoN<T> for Vec<T>
{
    fn into_1(self) -> T {
        let mut iter = self.into_iter();
        iter.next().unwrap()
    }
    fn into_2(self) -> (T, T) {
        let mut iter = self.into_iter();
        (iter.next().unwrap(), iter.next().unwrap())
    }
    fn into_3(self) -> (T, T, T) {
        let mut iter = self.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
    fn into_4(self) -> (T, T, T, T) {
        let mut iter = self.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
    
    fn try_into_1(self) -> Option<T> {
        let mut iter = self.into_iter();
        iter.next()
    }
    fn try_into_2(self) -> (Option<T>, Option<T>) {
        let mut iter = self.into_iter();
        (iter.next(), iter.next())
    }
    fn try_into_3(self) -> (Option<T>, Option<T>, Option<T>) {
        let mut iter = self.into_iter();
        (iter.next(), iter.next(), iter.next())
    }
    fn try_into_4(self) -> (Option<T>, Option<T>, Option<T>, Option<T>) {
        let mut iter = self.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next())
    }
    
}