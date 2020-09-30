use crate::nes::ppu::PPU;

// TODO: serde derive for saving
pub struct Console {
    cpu:       Box<CPU>,
    apu:       Box<APU>,
    ppu:       Box<PPU>,
    ctrler_1:  Box<Controller>,
    ctrler_2:  Box<Controller>,
    cartridge: Box<Cartridge>,
    mapper:    Box<dyn Mapper>,
    ram:       Vec<u8>,
}

impl Console {
    pub fn new(cartridge: Cartridge) -> Result<Console, ()> {
        let ram = [0; 2048].to_vec();
        let mapper = todo!();

        let console = Console {
            cpu: CPU::new(),
            apu: APU::new(),
            ppu: PPU::new(),
            ctrler_1: Controller::new(),
            ctrler_2: Controller::new(),
            cartridge,
            mapper,
            ram,
        }

        return Ok(console);
    }

    pub fn reset(&mut self) {
        self.cpu.reset()
    }

    pub fn step(&mut self) -> usize {
        let cpu_cycles = self.cpu.step();
        let ppu_cycles = cpu_cycles * 3;

        for i in 0..ppu_cycles {
            self.ppu.step();
            self.mapper.step();
        }

        for i in 0..cpu_cycles {
            self.apu.step()
        }

        return cpu_cycles;
    }

    pub fn step_frame(&mut self) -> usize {
        let mut cycles = 0;
        let frame = self.ppu.frame;

        while frame == self.ppu.frame {
            cycles += self.step();
        }

        return cycles
    }

    pub fn step_seconds(&mut self, seconds: f64) {
        let cycles = todo!(/* CPU Frequency */) * seconds;

        while cycles > 0 {
            cycles -= self.step();
        }
    }
}
