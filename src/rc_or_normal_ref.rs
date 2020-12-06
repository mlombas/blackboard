use std::rc::Rc;
use std::ops::Deref;

pub enum RcOrNormalRef<T: ?Sized> {
    Normal(Box<T>),
    Rc(Rc<T>),
}

use RcOrNormalRef::*;
impl<T: ?Sized> Deref for RcOrNormalRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Normal(inside) => inside,
            Rc(rc) => rc.deref(),
        }
    }
}

impl<T: ?Sized> RcOrNormalRef<T> {
    pub fn get_mut(&mut self) -> Option<&mut T> {
        match self {
            Normal(inside) => Some(inside),
            Rc(rc) => Rc::get_mut(rc),
        }
    }
}

impl<T: ?Sized> From<&T> for RcOrNormalRef<T> {
    fn from(what: T) -> RcOrNormalRef<T> {
        Normal(Box::new(what))
    }
}

impl<T: ?Sized> From<Box<T>> for RcOrNormalRef<T> {
    fn from(other: Box<T>) -> RcOrNormalRef<T> {
        Normal(other)
    }
}

impl<T: ?Sized> From<&Rc<T>> for RcOrNormalRef<T> {
    fn from(other: &Rc<T>) -> RcOrNormalRef<T> {
        Rc(Rc::clone(other))
    }
}
