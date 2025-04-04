use crate::dns::errors::*;
use std::ops::Deref;

const MAX_JUMP_ALLOWED: usize = 5;
pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl Deref for BytePacketBuffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.buf[..self.pos]
    }
}

impl BytePacketBuffer {
    pub fn new() -> Self {
        Self {
            buf: [0; 512],
            pos: 0,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn step(&mut self, steps: usize) -> DnsResult {
        self.pos += steps;
        Ok(())
    }

    pub fn seek(&mut self, pos: usize) -> DnsResult {
        self.pos = pos;

        Ok(())
    }

    pub fn read(&mut self) -> DnsResult<u8> {
        if self.pos >= 512 {
            return Err("read: End of buffer".into());
        }
        let res = self.buf[self.pos];
        self.pos += 1;
        Ok(res)
    }

    pub fn get(&self, pos: usize) -> DnsResult<u8> {
        if pos >= 512 {
            return Err("get: End of buffer".into());
        }
        Ok(self.buf[pos])
    }

    pub fn get_range(&self, start: usize, len: usize) -> DnsResult<&[u8]> {
        if start + len >= 512 {
            return Err("get_range: End of buffer".into());
        }
        Ok(&self.buf[start..start + len])
    }

    pub fn set(&mut self, pos: usize, val: u8) -> DnsResult {
        self.buf[pos] = val;

        Ok(())
    }
    pub fn set_u16(&mut self, pos: usize, val: u16) -> DnsResult {
        self.set(pos + 0, ((val >> 8) & 0xFF) as u8)?;
        self.set(pos + 1, ((val >> 0) & 0xFF) as u8)?;
        Ok(())
    }
    pub fn read_u16(&mut self) -> DnsResult<u16> {
        let res = ((self.read()? as u16) << 8) | (self.read()? as u16);
        Ok(res)
    }

    pub fn read_u32(&mut self) -> DnsResult<u32> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);
        Ok(res)
    }

    pub fn read_qname(&mut self, outstr: &mut String) -> DnsResult {
        let mut pos = self.pos();
        let mut jumped = false;
        let mut jump_performed = 0;
        let mut delim = "";

        loop {
            if jump_performed >= MAX_JUMP_ALLOWED {
                return Err("Limit of jumps exceeded".into());
            }

            let len = self.get(pos)?;
            if (len & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(pos + 2)?;
                }

                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;

                jumped = true;
                jump_performed += 1;
                continue;
            }

            pos += 1;
            if len == 0 {
                break;
            }
            outstr.push_str(delim);

            let buf_str = self.get_range(pos, len as usize)?;
            outstr.push_str(&String::from_utf8_lossy(buf_str).to_lowercase());

            delim = ".";
            pos += len as usize;
        } // next

        if !jumped {
            self.seek(pos)?;
        }

        Ok(())
    }

    pub fn write(&mut self, val: u8) -> DnsResult {
        if self.pos >= 512 {
            return Err("write: end of buffer".into());
        };
        self.buf[self.pos] = val;
        self.pos += 1;

        Ok(())
    }
    pub fn write_u8(&mut self, val: u8) -> DnsResult {
        self.write(val)
    }
    pub fn write_u16(&mut self, val: u16) -> DnsResult {
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write((val & 0xFF) as u8)?;

        Ok(())
    }
    pub fn write_u32(&mut self, val: u32) -> DnsResult {
        self.write(((val >> 24) & 0xFF) as u8)?;
        self.write(((val >> 16) & 0xFF) as u8)?;
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write(((val >> 0) & 0xFF) as u8)?;
        Ok(())
    }
    pub fn write_qname(&mut self, qname: &str) -> DnsResult {
        for label in qname.split(".") {
            let len = label.len();
            if len > 0x3f {
                return Err("single label exceeds 63 character of length".into());
            }
            self.write(len as u8)?;
            for b in label.as_bytes() {
                self.write(*b)?;
            }
        }
        self.write(0)?;
        Ok(())
    }
}
