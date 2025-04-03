use protocol_tls::{create_client_hello, create_paired_keys, TlsResult};
use std::io::{Read, Write};
use std::net::TcpStream;

const HOST: &str = "cloudflare.com";

fn main() -> TlsResult {
    let (_priv_key, pub_key) = create_paired_keys().expect("failed to create paired keys");

    let key = pub_key; //[32; 0];
    let pyl = create_client_hello(HOST.into(), key.as_ref());
    // let pyl = create_client_hello(HOST.into(), pub_key.as_ref());
    println!("pyl: {:0>2X?}", pyl);

    let mut stream = TcpStream::connect(format!("{HOST}:443")).expect("failed tcp stream");
    stream.write_all(&pyl).expect("failed to write_all");

    let mut buf = [0_u8; 512];
    let cnt = stream.read(&mut buf).expect("failed to read");
    let buf = &buf[..cnt];
    println!("received: {buf:0>2X?}");
    // let str = String::from_utf8_lossy(&buf);
    // println!("received: {str}");
    Ok(())
}

fn _main() -> TlsResult {
    println!("TLS 1.3 Protocol Implementation");

    let mut stream = TcpStream::connect("example.com:443").expect("failed tcp stream");

    #[rustfmt::skip]
    let client_hello_v =vec![
        // TLS Record Header
        0x16, 0x03, 0x01, 0x00, 0xF8,

        // Handshake Protocol Header
        0x01, 0x00, 0x00, 0xF4,

        // Client Version which is TLS 1.3 or SSL 3.3
        0x03, 0x03,

        // Random (32 bytes)
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,

        // Session ID (variable length; here, 1 byte, followed by the ID)
        0x20,
        0xE0, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7,
        0xE8, 0xE9, 0xEA, 0xEB, 0xEC, 0xED, 0xEE, 0xEF,
        0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7,
        0xF8, 0xF9, 0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xFF,

        // Cipher Suites
        0x00, 0x08,
        0x13, 0x02, 0x13, 0x03, 0x13, 0x01, 0x00, 0xFF,

        // Compression set to null (00)
        0x01, 0x00,

        // Extensions length 163
        0x00, 0xA3,

        // Ext. Server Name
        0x00, 0x00,
        0x00, 0x18, // len
        0x00, 0x16, 0x00, 0x00, 0x13, 0x65, 0x78, 0x61,
        0x6D, 0x70, 0x6C, 0x65, 0x2E, 0x75, 0x6C, 0x66,
        0x68, 0x65, 0x69, 0x6D, 0x2E, 0x6E, 0x65, 0x74,

        // Ext. EC Point Format
        0x00, 0x0B, 0x00, 0x04, 0x03, 0x00, 0x01, 0x02,

        // Ext. Supported Groups
        0x00, 0x0A, 0x00, 0x16, 0x00, 0x14, 0x00, 0x1D,
        0x00, 0x17, 0x00, 0x1E, 0x00, 0x19, 0x00, 0x18,
        0x01, 0x00, 0x01, 0x01, 0x01, 0x02, 0x01, 0x03,
        0x01, 0x04,

        // Ext. Session Ticket
        0x00, 0x23, 0x00, 0x00,

        // Ext. Encrypt Then MAC
        0x00, 0x16, 0x00, 0x00,
        // Ext. Extended Master Secret
        0x00, 0x17, 0x00, 0x00,

        // Ext. Signature Algorithms
        0x00, 0x0D,
        0x00, 0x1E, 0x00, 0x1C, 0x04, 0x03, 0x05, 0x03,
        0x06, 0x03, 0x08, 0x07, 0x08, 0x08, 0x08, 0x09,
        0x08, 0x0A, 0x08, 0x0B, 0x08, 0x04, 0x08, 0x05,
        0x08, 0x06, 0x04, 0x01, 0x05, 0x01, 0x06, 0x01,

        // Ext. Supported Version
        0x00, 0x2B, 0x00, 0x03, 0x02, 0x03, 0x04,

        // Ext. PSK Key Exchange Mode
        0x00, 0x2D, 0x00, 0x02, 0x01, 0x01,

        // Ext. Key Share Extensions
        0x00, 0x33, 0x00, 0x26, 0x00, 0x24, 0x00, 0x1D,
        0x00, 0x20, 0x35, 0x80, 0x72, 0xD6, 0x36, 0x58,
        0x80, 0xD1, 0xAE, 0xEA, 0x32, 0x9A, 0xDF, 0x91,
        0x21, 0x38, 0x38, 0x51, 0xED, 0x21, 0xA2, 0x8E,
        0x3B, 0x75, 0xE9, 0x65, 0xD0, 0xD2, 0xCD, 0x16,
        0x62, 0x54,
    ];
    // let str = String::from_utf8_lossy(&client_hello_v);
    // println!("sent: {str}");
    stream
        .write_all(&client_hello_v)
        .expect("failed to write_all");

    let mut buf = [0_u8; 512];
    let cnt = stream.read(&mut buf).expect("failed to read");
    let buf = &buf[..cnt];
    println!("received: {buf:x?}");
    let str = String::from_utf8_lossy(&buf);
    println!("received: {str}");
    Ok(())
}
