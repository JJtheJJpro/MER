pub struct ByteStream {
    buf: Vec<u8>,
    pub pos: usize,
}

impl ByteStream {
    pub fn new(buf: Vec<u8>) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn available(&self) -> bool {
        self.pos < self.buf.len()
    }

    pub fn check_reserved(&mut self, byte_count: usize) -> bool {
        let cp = self.pos;
        for _ in 0..byte_count {
            if self.buf[self.pos] != 0 {
                self.pos = cp + byte_count;
                return false;
            } else {
                self.pos += 1;
            }
        }
        true
    }

    pub fn read_byte(&mut self) -> u8 {
        let byte = self.buf[self.pos];
        self.pos += 1;
        byte
    }
    pub fn peek_byte(&mut self) -> u8 {
        self.buf[self.pos]
    }
    pub fn read_byte_at(&self, pos: usize) -> u8 {
        self.buf[pos]
    }
    pub fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut ret = Vec::new();
        for _ in 0..n {
            ret.push(self.buf[self.pos]);
            self.pos += 1;
        }
        ret
    }
    pub fn peek_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut ret = Vec::new();
        for i in 0..n {
            ret.push(self.buf[self.pos + i]);
        }
        ret
    }
    pub fn read_bytes_at(&self, n: usize, pos: usize) -> Vec<u8> {
        let mut ret = Vec::new();
        for i in 0..n {
            ret.push(self.buf[pos + i]);
        }
        ret
    }

    pub fn read_sbyte(&mut self) -> i8 {
        let byte = self.buf[self.pos] as i8;
        self.pos += 1;
        byte
    }

    pub fn read_word(&mut self) -> u16 {
        let b1 = self.read_byte() as u16;
        let b2 = self.read_byte() as u16;
        b2 << 8 | b1
    }
    pub fn peek_word(&self) -> u16 {
        ((self.buf[self.pos + 1] as u16) << 8) | (self.buf[self.pos] as u16)
    }
    pub fn read_word_at(&self, pos: usize) -> u16 {
        ((self.buf[pos + 1] as u16) << 8) | (self.buf[pos] as u16)
    }

    pub fn read_dword(&mut self) -> u32 {
        let w1 = self.read_word() as u32;
        let w2 = self.read_word() as u32;
        w2 << 16 | w1
    }

    pub fn read_string(&mut self, len: usize) -> String {
        String::from_utf8_lossy(&self.read_bytes(len)).to_string()
    }
    pub fn read_string_from_to(&self, from: usize, to: usize) -> String {
        String::from_utf8_lossy(&self.buf[from..to]).to_string()
    }

    pub fn replace_byte(&mut self, offset: usize, b: u8) {
        self.buf[offset] = b;
    }
    pub fn replace_word(&mut self, offset: usize, w: u16) {
        self.buf[offset] = (w & 0xFF) as u8;
        self.buf[offset + 1] = (w >> 8) as u8;
    }

    pub fn find_first_byte_from(&self, pos: usize, to_find: u8) -> usize {
        self.buf[pos..].binary_search(&to_find).unwrap() + pos
    }
}
