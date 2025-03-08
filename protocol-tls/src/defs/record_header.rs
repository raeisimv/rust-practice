use crate::defs::HandshakeType;
use crate::ProtocolVersion;

#[derive(Copy, Clone, Debug)]
pub struct RecordHeader {
    pub kind: HandshakeType,
    pub version: ProtocolVersion,
    pub size: u16,
}

impl Into<[u8; 5]> for RecordHeader {
    fn into(self) -> [u8; 5] {
        let mut v = [0; 5];
        let version: u16 = self.version.into();

        v[0] = self.kind.into();
        v[1] = ((version >> 8) & 0xFF) as u8;
        v[2] = ((version >> 0) & 0xFF) as u8;
        v[3] = ((self.size >> 8) & 0xFF) as u8;
        v[4] = ((self.size >> 0) & 0xFF) as u8;
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProtocolVersion::TLSv1_0;

    #[test]
    fn should_get_into_u8() {
        let x = RecordHeader {
            kind: HandshakeType::CertificateStatus,
            version: TLSv1_0,
            size: 248,
        };
        let bytes: [u8; 5] = x.into();

        assert_eq!(bytes, [0x16, 0x03, 0x01, 0x00, 0xF8]);
    }
}
