use crate::decode::Decoder;
use crate::rr::edns::{ExtendedDNSError, ExtendedDNSErrorKind};
use crate::{DecodeError, DecodeResult};
use std::str::from_utf8;
use std::convert::TryFrom;

impl<'a, 'b: 'a> Decoder<'a, 'b> {
    pub(super) fn rr_edns_extended_dns_error(&mut self) -> DecodeResult<ExtendedDNSError> {
        let info_code = self.u16()?;
        let vec = self.vec()?;
        match ExtendedDNSErrorKind::try_from(info_code) {
            Ok(kind) => Ok(ExtendedDNSError {
                kind,
                message: String::from(from_utf8(&vec)?),
            }),
            Err(code) => Err(DecodeError::ExtendedDNSError(code)),
        }
    }
}