pub trait AsBytes {
    fn as_bytes(&self) -> impl Iterator<Item = u8>;
}

pub trait IntoU8 {
    /// Separate integers into counterpart bytes
    /// ## Index
    /// The lower index the most right byte. With u16 the byte_at(0) is the right most byte, and byte_at(1) is the left most.
    /// The bigger index the Bigger the value worth
    ///
    /// ## Example
    /// ```rust
    /// use protocol_tls::IntoU8;
    ///
    /// let x = 0x0301_u16;
    /// assert_eq!(x.byte_at(1), 0x03);
    /// assert_eq!(x.byte_at(0), 0x01);
    /// ```
    fn byte_at(self, index: usize) -> u8;
}

impl IntoU8 for u16 {
    fn byte_at(self, index: usize) -> u8 {
        assert!(index < 2);

        (self >> (index * 8) & 0xff) as _
    }
}

impl IntoU8 for u32 {
    fn byte_at(self, index: usize) -> u8 {
        assert!(index < 4);

        (self >> (index * 8) & 0xff) as _
    }
}
