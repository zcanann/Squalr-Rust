use std::fmt;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Endian {
    Little,
    Big,
}

impl Default for Endian {
    fn default(
    ) -> Self {
        Endian::Little
    }
}

impl fmt::Display for Endian {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Endian::Little => write!(f, "le"),
            Endian::Big => write!(f, "be"),
        }
    }
}