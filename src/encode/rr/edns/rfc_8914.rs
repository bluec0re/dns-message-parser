use crate::encode::Encoder;
use crate::rr::edns::{ExtendedDNSError, EDNSOptionCode};
use crate::EncodeResult;

impl Encoder {
    pub(super) fn rr_edns_extended_dns_error(&mut self, error: &ExtendedDNSError) -> EncodeResult<()> {
        self.rr_edns_option_code(&EDNSOptionCode::ExtendedDNSError);
        self.u16(error.kind as u16);
        self.vec(error.message.as_bytes());
        Ok(())
    }
}
