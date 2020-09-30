// TODO: serde derive for save and load
pub struct PPU {
    // display
    cycle:    usize,
    scanline: usize,
    frame:    usize,

    // storage
    pallete:    [u8; 32],
    name_table: [u8; 2048],
    oam:        [u8; 256],
    front:      Box<Image>, // TODO: image
    back:       Box<Image>,

    // registers
    v: u16,
    t: u16,
    x: u8,
    w: u8,
    f: u8,

    register: u8,

    // NMI flags
    // TODO: wrap in struct?
    nmi: NMI,

    // background temps
    // TODO: better name than *_temp
    name_table_temp: u8,
    attr_table:      u8,
    low_tile:        u8,
    high_tile:       u8,
    tile:            u64,

    // sprite temps
    sprite: Sprite,

    // PPU flags
    control:  Control, // $2000
    mask:     Mask,    // $2001

    // $2002
    // TODO: maybe bytes?
    zero_hit: bool,
    overflow: bool,

    oam_addr: u8, // $2003
    buffered: u8, // $2007
}

struct NMI {
    // NMI flags
    occured:  bool,
    output:   bool,
    previous: bool,
    delay:    u8,
}

struct Sprite {
    count:      usize,
    patterns:   [u32; 8],
    positions:  [u8;  8],
    priorities: [u8;  8],
    indexes:    [u8;  8],
}

// TODO: better types?
struct Control {
    name_table:   u8,
    increment:    bool,
    sprite_table: bool,
    bg_table:     bool,
    sprite_size:  bool,
    master_slave: bool,
}

struct Mask {
    grayscale:    bool,
    left_bg:      bool,
    left_sprites: bool,
    bg:           bool,
    sprites:      bool,
    tint_red:     bool,
    tint_green:   bool,
    tint_blue:    bool,
}

impl PPU {
    pub fn new() -> PPU {
        todo!()
    }

    pub fn reset(&mut self) {
        self.cycle = 340;
        self.scanline = 240;
        self.frame = 0;
        // TODO remove 'write'?
        self.write_control(0);
        self.write_mask(0);
        self.write_oam_addr(0);
    }

    // TODO: implement body functions
}
