use std::fmt::{Display, Formatter, Result as FmtResult};

try_from_enum_to_integer! {
    #[repr(u16)]
    pub enum ExtendedDNSErrorKind {
        Other = 0,
        // ...
        RRSIGsMissing = 10,
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct ExtendedDNSError {
    kind: ExtendedDNSErrorKind,
    message: String,
}

impl Display for Padding {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Padding {}", self.0)
    }
}