
use crate::symbol::Symbol;

struct Scm {
    value: &'static ScmBox,
}

enum ScmBox {
    Nil,
    Integer(crate::config::Integer),
    Symbol(Symbol),
    Pair(Scm, Scm),
    Vector(Vec<Scm>),
}