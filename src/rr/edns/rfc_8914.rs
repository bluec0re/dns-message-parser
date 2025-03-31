use std::fmt::{Display, Formatter, Result as FmtResult};

try_from_enum_to_integer! {
    #[repr(u16)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum ExtendedDNSErrorKind {
        Other = 0,
        // ...
        RRSIGsMissing = 10,
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct ExtendedDNSError {
    pub kind: ExtendedDNSErrorKind,
    pub message: String,
}

impl Display for ExtendedDNSError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Extended DNS Error {:?}: {}", self.kind, self.message)
    }
}