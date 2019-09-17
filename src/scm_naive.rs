use crate::config;
use crate::memory::allocate;
use crate::symbol::Symbol;
use crate::StringInterner;

#[derive(Debug, Copy, Clone)]
pub struct Scm<E: 'static, F: 'static> {
    value: &'static ScmBox<E, F>,
}

pub enum ScmBox<E: 'static, F: 'static> {
    Uninitialized,
    Nil,
    True,
    False,
    Char(config::Char),
    Symbol(Symbol),
    Integer(config::Integer),
    Float(config::Float),
    Pair(Scm<E, F>, Scm<E, F>),
    Vector(Vec<Scm<E, F>>),
    String(config::String),

    Function(F),

    Primitive(fn(Scm<E, F>)->Result<Scm<E, F>, E>),
    Primitive0(fn()->Result<Scm<E, F>, E>),
    Primitive1(fn(Scm<E, F>)->Result<Scm<E, F>, E>),
    Primitive2(fn(Scm<E, F>, Scm<E, F>)->Result<Scm<E, F>, E>),
    Primitive3(fn(Scm<E, F>, Scm<E, F>, Scm<E, F>)->Result<Scm<E, F>, E>),

    PrimitiveWithState(Box<dyn Fn(Scm<E, F>)->Result<Scm<E, F>, E>>),
}

impl<E, F> Scm<E, F> {
    fn new(value: &'static ScmBox<E, F>) -> Self {
        Scm { value }
    }
}

impl<E: Clone, F: Clone> crate::SchemeValue for Scm<E, F> {
    type IntType = config::Integer;
    type FloatType = config::Float;
    type CharType = config::Char;

    // constructors
    fn nil() -> Self {
        Scm::new(&ScmBox::Nil)
    }

    fn bool(b: bool) -> Self {
        match b {
            true => Scm::new(&ScmBox::True),
            false => Scm::new(&ScmBox::False),
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

    fn float(r: Self::FloatType) -> Self {
        Scm::new(allocate(ScmBox::Float(r)))
    }

    fn cons(car: Self, cdr: Self) -> Self {
        Scm::new(allocate(ScmBox::Pair(car, cdr)))
    }

    fn vect(size: usize) -> Self {
        Scm::new(allocate(ScmBox::Vector(vec![
            Scm::new(&ScmBox::Uninitialized);
            size
        ])))
    }

    fn string<T: ToString>(s: T) -> Self {
        Scm::new(allocate(ScmBox::String(s.to_string())))
    }
}

impl<E, F> std::fmt::Debug for ScmBox<E, F> {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unimplemented!()
    }
}