// A full definitions of these enums can be found at:
// https://github.com/rustls/rustls/blob/5860d10317528e4f162db6e26c74f81575c51403/rustls/src/enums.rs

#[derive(Copy, Clone, Debug)]
pub enum HandshakeType {
    CertificateStatus,
    Unsupported(u8),
}

impl Into<u8> for HandshakeType {
    fn into(self) -> u8 {
        match self {
            HandshakeType::CertificateStatus => 0x16,
            HandshakeType::Unsupported(x) => x,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ProtocolVersion {
    TLSv1_0,
    TLSv1_1,
    TLSv1_2,
    TLSv1_3,
    Unsupported(u16),
}
impl Into<u16> for ProtocolVersion {
    fn into(self) -> u16 {
        match self {
            ProtocolVersion::TLSv1_0 => 0x0301,
            ProtocolVersion::TLSv1_1 => 0x0302,
            ProtocolVersion::TLSv1_2 => 0x0303,
            ProtocolVersion::TLSv1_3 => 0x0304,
            ProtocolVersion::Unsupported(x) => x,
        }
    }
}
