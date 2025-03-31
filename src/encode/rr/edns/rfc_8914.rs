use crate::encode::Encoder;
use crate::rr::edns::{ExtendedDNSError, EDNSOptionCode};
use crate::EncodeResult;

impl Encoder {
    pub(super) fn rr_edns_cookie(&mut self, cookie: &Cookie) -> EncodeResult<()> {
        self.rr_edns_option_code(&EDNSOptionCode::ExtendedDNSError);
        self.u16(self.kind as u16)?;
        self.vec(self.message.as_bytes())?;
        Ok(())
    }
}
