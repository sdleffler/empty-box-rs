//! # `EmptyBox`, a way to safely move values in and out of `Box`s without
//! reallocations
//!
//! `EmptyBox` is similar to a statically checked `Box<Option<T>>`:
//!
//! ```
//! use empty_box::EmptyBox;
//!
//! // A box with a string!
//! let boxed = Box::new("Hello!".to_string());
//!
//! // Oh no, we don't like that string.
//! let (string, empty) = EmptyBox::take(boxed);
//!
//! // Let's make an objectively superior string, and put it into the original
//! // box.
//! let superior = "Objectively superior string!".to_string();
//!
//! // Now we have our superior string in the box!
//! let boxed = empty.put(superior); 
//!
//! assert_eq!("Hello!", string);
//! assert_eq!("Objectively superior string!", &*boxed);
//! ```
//!
//! Creating an `EmptyBox` from a `Box` and then putting a `T` back into the
//! `EmptyBox` will avoid allocating a new `Box`, instead reusing whatever old
//! `Box` the `T` was `EmptyBox::take`n from.

use std::mem;
use std::ptr;


/// An "emptied" `Box`. Constructed via `EmptyBox::take()`, an `EmptyBox<T>` is
/// a `Box` from which the contents have been moved. This allows for reuse of the
/// `Box` via `EmptyBox::put()`, which moves the contents back in, turning the
/// `EmptyBox` back into a `Box<T>`.
pub struct EmptyBox<T> {
    ptr: *mut T,
}


impl<T> Drop for EmptyBox<T> {
    fn drop(&mut self) {
        let boxed = unsafe { Box::from_raw(self.ptr) };
        let inner = *boxed;
        mem::forget(inner);
    }
}


impl<T> EmptyBox<T> {
    /// Move the value out of the `Box`, creating a `T` and an `EmptyBox` which
    /// preserves the original `Box`'s allocation.
    pub fn take(bx: Box<T>) -> (T, EmptyBox<T>) {
        let ptr = Box::into_raw(bx);
        let t = unsafe { ptr::read(ptr) };
        (t, EmptyBox { ptr })
    }


    /// Restore a value to an `EmptyBox`, creating a new `Box` and reusing the
    /// allocation of whatever `Box` was destroyed to create the `EmptyBox`.
    pub fn put(self, t: T) -> Box<T> {
        let ptr = self.ptr;
        mem::forget(self);

        unsafe {
            ptr::write(ptr, t);
            Box::from_raw(ptr)
        }
    }
}


#[cfg(test)]
mod test {
    use std::cell::Cell;

    use super::*;


    #[derive(Clone)]
    pub struct DropCounter<'a>(&'a Cell<usize>);

    impl<'a> Drop for DropCounter<'a> {
        fn drop(&mut self) {
            let prev = self.0.get();
            self.0.set(prev + 1);
        }
    }


    #[test]
    fn drop_counter() {
        let counter = Cell::new(0);

        mem::drop(DropCounter(&counter));
        mem::drop(DropCounter(&counter));

        let dc = DropCounter(&counter);

        assert_eq!(counter.get(), 2);

        mem::drop(dc);

        assert_eq!(counter.get(), 3);
    }


    #[test]
    fn no_drop() {
        let counter = Cell::new(0);

        let dc = {
            let boxed = Box::new(DropCounter(&counter));
            EmptyBox::take(boxed).0
        };

        assert_eq!(counter.get(), 0);

        mem::drop(dc);
    }


    #[test]
    fn two_drop() {
        let counter = Cell::new(0);

        let boxed = Box::new(DropCounter(&counter));
        let (dc, empty) = EmptyBox::take(boxed);

        mem::drop(dc);

        mem::drop(empty.put(DropCounter(&counter)));

        assert_eq!(counter.get(), 2);
    }
}
