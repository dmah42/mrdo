use std::fmt;

use nom::error::ErrorKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Builtin {
    Write,
    Map,
    Filter,
    Fold,
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<'a> TryFrom<&'a str> for Builtin {
    type Error = nom::error::Error<&'a str>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        for builtin in iterator() {
            if s.to_lowercase() == builtin.to_string().to_lowercase() {
                return Ok(builtin);
            }
        }
        Err(nom::error::Error::new(s, ErrorKind::IsNot))
    }
}

fn iterator() -> impl Iterator<Item = Builtin> {
    [Builtin::Write, Builtin::Map, Builtin::Filter, Builtin::Fold]
        .iter()
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    use nom::error::Error;

    #[test]
    fn test_to_string() {
        assert_eq!(Builtin::Write.to_string(), "Write");
    }

    #[test]
    fn test_try_from() {
        assert_eq!(Builtin::try_from("write"), Ok(Builtin::Write));
        assert_eq!(Builtin::try_from("wRiTe"), Ok(Builtin::Write));
        assert_eq!(
            Builtin::try_from("foo"),
            Err(Error::new("foo", ErrorKind::IsNot))
        );
    }

    #[test]
    fn test_try_into() {
        let result: Result<Builtin, Error<&str>> = "write".try_into();
        assert_eq!(result, Ok(Builtin::Write));
        let result: Result<Builtin, Error<&str>> = "foo".try_into();
        assert_eq!(result, Err(Error::new("foo", ErrorKind::IsNot)));
    }
}
