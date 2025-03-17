// A full definitions of these enums can be found at:
// https://github.com/rustls/rustls/blob/5860d10317528e4f162db6e26c74f81575c51403/rustls/src/enums.rs

#[derive(Copy, Clone, Debug)]
pub enum HandshakeType {
    HelloRequest,
    ClientHello,
    ServerHello,
    HelloVerifyRequest,
    NewSessionTicket,
    EndOfEarlyData,
    CertificateStatus,

    // Categorize others as unsupported for now
    Unsupported(u8),
}

impl Into<u8> for HandshakeType {
    fn into(self) -> u8 {
        match self {
            HandshakeType::HelloRequest => 0x00,
            HandshakeType::ClientHello => 0x01,
            HandshakeType::ServerHello => 0x02,
            HandshakeType::HelloVerifyRequest => 0x03,
            HandshakeType::NewSessionTicket => 0x04,
            HandshakeType::EndOfEarlyData => 0x05,
            HandshakeType::CertificateStatus => 0x16,
            HandshakeType::Unsupported(x) => x,
        }
    }
}

enum_builder! {
    #[repr(u16)]
    #[derive(Clone,Copy, Eq, PartialEq, Debug)]
    pub enum ProtocolVersion {
        SSLv2 => 0x0002,
        SSLv3 => 0x0300,
        TLSv1_0 => 0x0301,
        TLSv1_1 => 0x0302,
        TLSv1_2 => 0x0303,
        TLSv1_3 => 0x0304,
        DTLSv1_0 => 0xFEFF,
        DTLSv1_2 => 0xFEFD,
        DTLSv1_3 => 0xFEFC,
    }
}

enum_builder! {
    #[repr(u16)]
    #[allow(non_camel_case_types)]
    #[derive(Clone,Copy, Eq, PartialEq, Debug)]
    pub enum CipherSuite {
        TLS_AES_256_GCM_SHA384  => 0x1302,
        TLS_CHACHA20_POLY1305_SHA256 => 0x1303,
        TLS_AES_128_GCM_SHA256=> 0x1301,
        TLS_EMPTY_RENEGOTIATION_INFO_SCSV => 0x00ff,
    }
}
