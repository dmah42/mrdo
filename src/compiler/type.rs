use std::fmt;

use nom::error::ErrorKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    Real,
    Integer,
    Coll,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<'a> TryFrom<&'a str> for Type {
    type Error = nom::error::Error<&'a str>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        for typ in iterator() {
            if s.to_lowercase() == typ.to_string().to_lowercase() {
                return Ok(typ);
            }
        }
        Err(nom::error::Error::new(s, ErrorKind::IsNot))
    }
}

fn iterator() -> impl Iterator<Item = Type> {
    [Type::Real, Type::Integer, Type::Coll].iter().copied()
}
