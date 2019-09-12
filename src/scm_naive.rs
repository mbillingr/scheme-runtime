use crate::config;
use crate::memory::allocate;
use crate::symbol::Symbol;
use crate::StringInterner;

const UNINITIALIZED: ScmBox = ScmBox::Uninitialized;
const NIL: ScmBox = ScmBox::Nil;
const TRUE: ScmBox = ScmBox::True;
const FALSE: ScmBox = ScmBox::False;

#[derive(Debug, Copy, Clone)]
pub struct Scm {
    value: &'static ScmBox,
}

#[derive(Debug)]
pub enum ScmBox {
    Uninitialized,
    Nil,
    True,
    False,
    Char(config::Char),
    Symbol(Symbol),
    Integer(config::Integer),
    Pair(Scm, Scm),
    Vector(Vec<Scm>),
    String(config::String),
}

impl Scm {
    fn new(value: &'static ScmBox) -> Self {
        Scm { value }
    }
}

impl crate::SchemeValue for Scm {
    type IntType = config::Integer;
    type CharType = config::Char;

    // constructors
    fn nil() -> Self {
        Scm::new(&NIL)
    }

    fn bool(b: bool) -> Self {
        match b {
            true => Scm::new(&TRUE),
            false => Scm::new(&FALSE),
        }
    }

    fn char(ch: Self::CharType) -> Self {
        Scm::new(allocate(ScmBox::Char(ch)))
    }

    fn symbol<T: StringInterner>(s: T) -> Self {
        Scm::new(allocate(ScmBox::Symbol(Symbol::new(s))))
    }

    fn int(i: Self::IntType) -> Self {
        Scm::new(allocate(ScmBox::Integer(i)))
    }

    fn cons(car: Self, cdr: Self) -> Self {
        Scm::new(allocate(ScmBox::Pair(car, cdr)))
    }

    fn vect(size: usize) -> Self {
        Scm::new(allocate(ScmBox::Vector(vec![
            Scm::new(&UNINITIALIZED);
            size
        ])))
    }

    fn string<T: ToString>(s: T) -> Self {
        Scm::new(allocate(ScmBox::String(s.to_string())))
    }
}
