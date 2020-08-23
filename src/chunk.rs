#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<f64>,
    pub lines: Vec<u64>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }
    pub fn write_chunk(&mut self, byte: u8, line: u64) {
        self.code.push(byte);
        self.lines.push(line);
    }
    pub fn add_constant(&mut self, value: f64) -> usize {
        self.constants.push(value);
        return self.constants.len() - 1;
    }
}
