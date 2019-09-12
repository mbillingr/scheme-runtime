
#[derive(Debug, Copy, Clone)]
pub struct Symbol {
    name: &'static str,
}

trait StringInterner {
    /// if a string with the same name has been interned return it,
    /// otherwise call statify, intern the result and return it.
    fn interned(self) -> &'static str;

    /// convert self into a static string slice
    fn statify(self) -> &'static str;
}

/// Symbols are named entities. They are interned as static string slices and compared by address.
/// TODO: make sure the garbage collector can see the collection of interned strings.
impl Symbol {
    /// Get interned symbol with given name.
    fn new<T: StringInterner>(name: T) -> Self {
        Symbol {
            name: name.interned()
        }
    }

    /// Create an uninterned symbol.
    /// Uninterned symbols usually do not compare equal with any other symbol.
    /// However, if created from the same string slice, even these symbols will
    /// compare equal.
    fn new_uninterned<T: StringInterner>(name: T) -> Self {
        Symbol {
            name: name.statify()
        }
    }
}

impl PartialEq for Symbol {
    fn eq(&self, rhs: &Self) -> bool {
        std::ptr::eq(self.name, rhs.name)
    }
}

impl StringInterner for &'static str {
    fn interned(self) -> &'static str {
        interner_impl::get_interned_string(self).unwrap_or_else(|| interner_impl::set_interned_string(self))
    }

    fn statify(self) -> &'static str {
        self
    }
}

impl StringInterner for String {
    fn interned(self) -> &'static str {
        interner_impl::get_interned_string(&self).unwrap_or_else(||{
            interner_impl::set_interned_string(self.statify())
        })
    }

    fn statify(self) -> &'static str {
        Box::leak(self.into_boxed_str())
    }
}

impl StringInterner for &String {
    fn interned(self) -> &'static str {
        interner_impl::get_interned_string(&self).unwrap_or_else(||{
            interner_impl::set_interned_string(self.statify())
        })
    }

    fn statify(self) -> &'static str {
        String::statify(self.clone())
    }
}

#[cfg(feature = "multi-threaded")]
mod interner_impl {
    use std::collections::HashSet;
    use lazy_static::lazy_static;
    use crate::symbol::StringInterner;
    use std::sync::RwLock;

    lazy_static! {
        static ref INTERNED_STRINGS: RwLock<HashSet<&'static str>> = RwLock::new(HashSet::new());
    }

    pub fn get_interned_string(s: &str) -> Option<&'static str> {
        INTERNED_STRINGS.read().unwrap().get(s).copied()
    }

    pub fn set_interned_string(s: &'static str) -> &'static str {
        INTERNED_STRINGS.write().unwrap().insert(s);
        s
    }
}

#[cfg(feature = "single-threaded")]
mod interner_impl {
    use std::collections::HashSet;
    use lazy_static::lazy_static;
    use crate::symbol::StringInterner;
    use std::cell::UnsafeCell;

    thread_local! {
        static INTERNED_STRINGS: UnsafeCell<HashSet<&'static str>> = UnsafeCell::new(HashSet::new());
    }

    pub fn get_interned_string(s: &str) -> Option<&'static str> {
        INTERNED_STRINGS.with(|is|unsafe {
            (*is.get()).get(s).copied()
        })
    }

    pub fn set_interned_string(s: &'static str) -> &'static str {
        INTERNED_STRINGS.with(|is|unsafe {
            (*is.get()).insert(s)
        });
        s
    }


}

#[cfg(test)]
mod tests {
    use super::Symbol;
    use super::StringInterner;
    use std::ptr;

    #[test]
    fn intern_same_static_str() {
        let a = "foo".interned();
        let b = "foo".interned();
        assert!(ptr::eq(a, b));
    }

    #[test]
    fn intern_different_static_str() {
        let a = "boo".interned();
        let b = "bar".interned();
        assert!(!ptr::eq(a, b));
    }

    #[test]
    fn intern_same_str_different_addr() {
        let a = "baz".to_owned().interned();
        let b = "baz".to_owned().interned();
        assert!(ptr::eq(a, b));
    }

    #[test]
    fn interned_symbols() {
        let a = Symbol::new("a");
        let b1 = Symbol::new("b".to_owned());
        let b2 = Symbol::new(&"b".to_owned());
        assert_ne!(a, b1);
        assert_ne!(a, b2);
        assert_eq!(b1, b2);
    }

    #[test]
    fn uninterned_symbols() {
        let a = Symbol::new("c");
        let b1 = Symbol::new_uninterned("c".to_owned());
        let b2 = Symbol::new_uninterned("c".to_owned());
        assert_ne!(a, b1);
        assert_ne!(a, b2);
        assert_ne!(b1, b2);
    }
}
