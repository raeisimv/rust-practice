// A full definitions of these enums can be found at:
// https://github.com/rustls/rustls/blob/5860d10317528e4f162db6e26c74f81575c51403/rustls/src/enums.rs
enum_builder! {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug)]
    pub enum HandshakeType {
        HelloRequest  => 0x00,
        ClientHello=> 0x01,
        ServerHello => 0x02,
        HelloVerifyRequest => 0x03,
        NewSessionTicket => 0x04,
        EndOfEarlyData => 0x05,
        CertificateStatus => 0x16,
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

enum_builder! {
    #[repr(u16)]
    #[derive(Clone, Copy, Debug)]
    pub enum ExtensionType {
        ServerName => 0x0000,
        ECPointFormats => 0x000b,
        EllipticCurves => 0x000a,
        SessionTicket => 0x0023,
        EncryptThenMAC => 0x0016,
        ExtendedMasterSecret => 0x0017,
        SignatureAlgorithms => 0x000d,
        SupportedVersions => 0x002b,
        PSKKeyExchangeModes => 0x002d,
        KeyShare => 0x0033,
    }
}

enum_builder! {
    #[repr(u16)]
    #[derive(Clone, Copy, Debug)]
    pub enum NamedGroup {
        X25519 => 0x001d,
    }
}

enum_builder! {
    #[repr(u16)]
    #[allow(non_camel_case_types)]
    #[derive(Clone,Copy, Eq, PartialEq, Debug)]
    pub enum SignatureScheme {
        RSA_PKCS1_SHA1 => 0x0201,
        ECDSA_SHA1_Legacy => 0x0203,
        RSA_PKCS1_SHA256 => 0x0401,
        ECDSA_NISTP256_SHA256 => 0x0403,
        RSA_PKCS1_SHA384 => 0x0501,
        ECDSA_NISTP384_SHA384 => 0x0503,
        RSA_PKCS1_SHA512 => 0x0601,
        ECDSA_NISTP521_SHA512 => 0x0603,
        RSA_PSS_SHA256 => 0x0804,
        RSA_PSS_SHA384 => 0x0805,
        RSA_PSS_SHA512 => 0x0806,
        ED25519 => 0x0807,
        ED448 => 0x0808,
    }
}
