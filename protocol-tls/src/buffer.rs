pub struct BufReader<'a> {
    buf: &'a [u8],
    pos: usize,
}
impl<'a> BufReader<'a> {
    /// Init a new buffer reader from the given slice binding the lifetime
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// How many bytes left in the buffer to read
    pub fn left(&self) -> usize {
        self.buf.len() - self.pos
    }

    /// Take out bytes from the buffer if left > size
    pub fn take(&mut self, size: usize) -> Option<&'a [u8]> {
        if self.left() < size {
            return None;
        }
        let taken = &self.buf[self.pos..self.pos + size];
        self.pos += size;

        Some(taken)
    }
}
