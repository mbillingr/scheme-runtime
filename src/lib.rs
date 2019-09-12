mod config;
mod memory;
pub mod scm_naive;
mod symbol;

pub use symbol::{StringInterner, Symbol};

trait SchemeValue {
    type IntType;
    type CharType;

    // constructors
    fn nil() -> Self;
    fn bool(b: bool) -> Self;
    fn char(ch: Self::CharType) -> Self;
    fn symbol<T: StringInterner>(s: T) -> Self;
    fn int(i: Self::IntType) -> Self;

    fn cons(car: Self, cdr: Self) -> Self;
    fn vect(size: usize) -> Self;
    fn string<T: ToString>(s: T) -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
