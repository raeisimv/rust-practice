use crate::dns::errors::*;

const MAX_JUMP_ALLOWED: usize = 5;
pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
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

    pub fn step(&mut self, steps: usize) -> Result {
        self.pos += steps;
        Ok(())
    }

    pub fn seek(&mut self, pos: usize) -> Result {
        self.pos = pos;

        Ok(())
    }

    pub fn read(&mut self) -> Result<u8> {
        if self.pos >= 512 {
            return Err("end of buffer".into());
        }
        let tmp = self.buf[self.pos];
        self.step(1)?;

        Ok(tmp)
    }

    pub fn get(&self, pos: usize) -> Result<u8> {
        if pos > 512 {
            return Err("End of buffer".into());
        }
        Ok(self.buf[pos])
    }

    pub fn get_range(&self, start: usize, len: usize) -> Result<&[u8]> {
        if start + len >= 512 {
            return Err("End of buffer".into());
        }
        Ok(&self.buf[start..start + len])
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let res = ((self.read()? as u16) << 8) | ((self.read()? as u16) << 0);
        Ok(res)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);
        Ok(res)
    }

    pub fn read_qname(&mut self, outstr: &mut String) -> Result {
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
                let offset = (((len as u16) & 0xC0) << 8) | b2;
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
}
