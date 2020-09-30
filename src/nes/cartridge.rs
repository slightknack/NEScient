// TODO: serde derive for saving and loading
pub struct Cartridge {
    prg:     Vec<u8>,
    chr:     Vec<u8>,
    sram:    Vec<u8>,
    mapper:  u8, // TODO: enum?
    mirror:  u8, // mirroring mode
    battery: bool,
}

impl Cartridge {
    pub fn new(
        prg: Vec<u8>,
        chr: Vec<u8>,
        mapper: u8,
        mirror: u8,
        battery: bool,
    ) -> Cartridge {
        let sram = [0; 0x2000].to_vec();
        Cartridge {
            prg,    chr,    sram,
            mapper, mirror, battery,
        }
    }
}
