pub struct Ram {
    mem: [u8; 4096],
}

impl Ram {

    pub fn new() -> Ram {
        let mut ram = Ram {
            mem: [0; 4096],
        };

        ram.init_sprites();

        ram
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    fn init_sprites(&mut self) {
        //values 0x000 to 0x1FF are reserved for the interpreter
        //Load the 'art' for 0
        self.write_byte(0x000, 0xF0);
        self.write_byte(0x001, 0x90);
        self.write_byte(0x002, 0x90);
        self.write_byte(0x003, 0x90);
        self.write_byte(0x004, 0xF0);

        //'art' for 1
        self.write_byte(0x005, 0x20);
        self.write_byte(0x006, 0x60);
        self.write_byte(0x007, 0x20);
        self.write_byte(0x008, 0x20);
        self.write_byte(0x009, 0x70);

        //'art' for 2
        self.write_byte(0x00A, 0xF0);
        self.write_byte(0x00B, 0x10);
        self.write_byte(0x00C, 0xF0);
        self.write_byte(0x00D, 0x80);
        self.write_byte(0x00E, 0xF0);

        //'art' for 3
        self.write_byte(0x00F, 0xF0);
        self.write_byte(0x010, 0x10);
        self.write_byte(0x011, 0xF0);
        self.write_byte(0x012, 0x10);
        self.write_byte(0x013, 0xF0);

        //'art' for 4
        self.write_byte(0x014, 0x90);
        self.write_byte(0x015, 0x90);
        self.write_byte(0x016, 0xF0);
        self.write_byte(0x017, 0x10);
        self.write_byte(0x018, 0x10);

        //'art' for 5
        self.write_byte(0x019, 0xF0);
        self.write_byte(0x01A, 0x80);
        self.write_byte(0x01B, 0xF0);
        self.write_byte(0x01C, 0x10);
        self.write_byte(0x01D, 0xF0);

        //'art' for 6
        self.write_byte(0x01E, 0xF0);
        self.write_byte(0x01F, 0x80);
        self.write_byte(0x020, 0xF0);
        self.write_byte(0x021, 0x90);
        self.write_byte(0x022, 0xF0);

        //'art' for 7
        self.write_byte(0x023, 0xF0);
        self.write_byte(0x024, 0x10);
        self.write_byte(0x025, 0x20);
        self.write_byte(0x026, 0x40);
        self.write_byte(0x027, 0x40);

        //'art' for 8
        self.write_byte(0x028, 0xF0);
        self.write_byte(0x029, 0x90);
        self.write_byte(0x02A, 0xF0);
        self.write_byte(0x02B, 0x90);
        self.write_byte(0x02C, 0xF0);

        //'art' for 9
        self.write_byte(0x02D, 0xF0);
        self.write_byte(0x02E, 0x90);
        self.write_byte(0x02F, 0xF0);
        self.write_byte(0x030, 0x10);
        self.write_byte(0x031, 0xF0);

        //'art' for A
        self.write_byte(0x032, 0xF0);
        self.write_byte(0x033, 0x90);
        self.write_byte(0x034, 0xF0);
        self.write_byte(0x035, 0x90);
        self.write_byte(0x036, 0x90);

        //'art' for B
        self.write_byte(0x037, 0xE0);
        self.write_byte(0x038, 0x90);
        self.write_byte(0x039, 0xE0);
        self.write_byte(0x03A, 0x90);
        self.write_byte(0x03B, 0xE0);

        //'art' for C
        self.write_byte(0x03C, 0xF0);
        self.write_byte(0x03D, 0x80);
        self.write_byte(0x03E, 0x80);
        self.write_byte(0x03F, 0x80);
        self.write_byte(0x040, 0xF0);

        //'art' for D
        self.write_byte(0x041, 0xE0);
        self.write_byte(0x042, 0x90);
        self.write_byte(0x043, 0x90);
        self.write_byte(0x044, 0x90);
        self.write_byte(0x045, 0xE0);

        //'art' for E
        self.write_byte(0x046, 0xF0);
        self.write_byte(0x047, 0x80);
        self.write_byte(0x048, 0xF0);
        self.write_byte(0x049, 0x80);
        self.write_byte(0x04A, 0xF0);

        //'art' for F
        self.write_byte(0x04B, 0xF0);
        self.write_byte(0x04C, 0x80);
        self.write_byte(0x04D, 0xF0);
        self.write_byte(0x04E, 0x80);
        self.write_byte(0x04F, 0x80);
    }
}