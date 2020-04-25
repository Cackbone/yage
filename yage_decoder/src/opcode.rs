pub struct Opcode {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub p: u8,
    pub q: u8
}


impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        // Bits 7-6
        let x = opcode >> 6;
        // Bits 5-3
        let y = 0b111 & (opcode >> 3);
        // Bits 2-0
        let z = 0b111 & opcode;
        // Bits 5-4
        let p = y >> 1;
        // Bit 3
        let q = y % 2;

        Opcode { x, y, z, p, q }
    }
}
