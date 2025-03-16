use crate::HandshakeType;
use crate::ProtocolVersion;

#[derive(Copy, Clone, Debug)]
pub struct RecordHeader {
    pub kind: HandshakeType,
    pub version: ProtocolVersion,
    pub size: u16,
}

impl Into<[u8; 5]> for RecordHeader {
    fn into(self) -> [u8; 5] {
        let version = u16::from(self.version).to_be_bytes();
        let size = self.size.to_be_bytes();
        [self.kind.into(), version[0], version[1], size[0], size[1]]
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
