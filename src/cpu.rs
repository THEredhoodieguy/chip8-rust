use rand;
use rand::distributions::{IndependentSample, Range};

use crate::bus::Bus;

pub const PROGRAM_START : u16 = 0x200;

pub struct CPU {
    vx: [u8; 16],
    i: u16,
    sound: u8,
    delay: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    rng: rand::ThreadRng,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            vx: [0; 16],
            i: 0,
            sound: 0,
            delay: 0,
            pc: PROGRAM_START,
            sp: 0,
            stack: [0; 16],
            rng: rand::thread_rng(),
        }
        
    }

    pub fn run_instruction(&mut self, bus: &mut Bus) {
        let hi: u8 = bus.ram_read_byte(self.pc);
        let low: u8 = bus.ram_read_byte(self.pc + 1);

        let instruction: u16 = (hi as u16) << 8 | (low as u16);


        //println!("Instruction read: {:#X} hi:{:#X} low:{:#X}", instruction, hi, low);
        //println!("Instruction after operation: {:#X}", (instruction & 0xF000) >> 12);
        if instruction == 0 {
            panic!();
        }

        let nnn = instruction & 0x0FFF;
        let kk  = (instruction & 0x0FF) as u8;
        let n   = (instruction & 0x00F) as u8;
        let x   = ((instruction & 0x0F00) >> 8) as u8;
        let y   = ((instruction & 0x00F0) >> 4) as u8;

        match (instruction & 0xF000) >> 12 {
            0x0 => {
                match instruction {
                    0x00E0 => {
                        //CLS
                        //clear the screen
                        bus.clear_screen();
                        self.pc += 2;
                    }
                    0x00EE => {
                        //RET
                        self.pc = self.stack[self.sp as usize];
                        self.sp -= 1;
                    }
                    _ => panic!(
                        "0x00 Instruction not recognized: {:#X}", 
                        instruction
                        )
                }
            }
            0x1 => {
                //JP addr
                self.pc = nnn;
            }
            0x2 => {
                //CALL addr
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc + 2;
                self.pc = nnn;
            }
            0x3 => {
                //SE Vx, byte
                if self.vx[x as usize] == kk {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x4 => {
                //SNE Vx, byte
                if self.vx[x as usize] != kk {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x5 => {
                //SE Vx, Vy
                if self.vx[x as usize] == self.vx[y as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x6 => {
                //LD Vx, byte
                self.vx[x as usize] = kk;
                self.pc += 2;
            }
            0x7 => {
                //ADD Vx, byte
                self.vx[x as usize] = self.vx[x as usize].wrapping_add(kk);
                self.pc += 2;
            }
            0x8 => {
                match n {
                    0x0 => {
                        //LD Vx, Vy
                        self.vx[x as usize] = self.vx[y as usize];
                    }
                    0x1 => {
                        //OR Vx, Vy
                        self.vx[x as usize] = self.vx[x as usize] | self.vx[y as usize];
                    }
                    0x2 => {
                        //AND Vx, Vy
                        self.vx[x as usize] = self.vx[x as usize] & self.vx[y as usize];
                    }
                    0x3 => {
                        //XOR Vx, Vy
                        self.vx[x as usize] = self.vx[x as usize] ^ self.vx[y as usize];
                    }
                    0x4 => {
                        //ADD Vx, Vy
                        let sum: u16 = self.vx[x as usize] as u16 + self.vx[y as usize] as u16;
                        self.vx[x as usize] = sum as u8;
                        if sum > 0xFF {
                            self.vx[0xF as usize] = 1;
                        }
                    }
                    0x5 => {
                        //SUB Vx, Vy
                        if self.vx[x as usize] > self.vx[y as usize] {
                            self.vx[0xF as usize] = 1;
                        } else {
                            self.vx[0xF as usize] = 0;
                        }
                        self.vx[x as usize] = self.vx[x as usize] - self.vx[y as usize];
                    }
                    0x6 => {
                        //SHR Vx {, Vy}
                        self.vx[0xF as usize] = self.vx[x as usize] & 0x1;
                        self.vx[x as usize] = self.vx[x as usize] << 1;
                    }
                    0x7 => {
                        //SUBN Vx, Vy
                        if self.vx[y as usize] > self.vx[x as usize] {
                            self.vx[0xF as usize] = 1;
                        } else {
                            self.vx[0xF as usize] = 0;
                        }
                        self.vx[x as usize] = self.vx[y as usize] - self.vx[x as usize];
                    }
                    0xE => {
                        //SHL Vx {, Vy}
                        self.vx[0xF as usize] = self.vx[x as usize] >> 7;
                        self.vx[x as usize] = self.vx[x as usize] << 1;
                    }
                    _ => panic!(" 0x8XY* Instruction not recognized: {:#X}", instruction)
                }
                self.pc += 2;
            }
            0x9 => {
                //SNE Vx, Vy
                if self.vx[x as usize] != self.vx[y as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0xA => {
                //LD I, addr
                self.i = nnn;
                self.pc += 2;
            }
            0xB => {
                //JP V0, addr
                self.pc = nnn + self.vx[0x0 as usize] as u16;
            }
            0xC => {
                //RND Vx, byte
                let interval = Range::new(0, 255);
                let number = interval.ind_sample(&mut self.rng);
                self.vx[x as usize] = number & kk;
                self.pc += 2;
            }
            0xD => {
                //DRW Vx, Vy, nibble
                self.debug_draw_sprite(bus, n, self.vx[x as usize], self.vx[y as usize]);
                self.pc += 2;
            }
            0xE => {
                match kk {
                    0x9E => {
                        //SKP Vx
                        //TODO
                        let key = self.vx[x as usize];
                        if bus.is_key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        //SKNP Vx
                        let key = self.vx[x as usize];
                        if !bus.is_key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    _ => panic!("0xE instruction not recognized: {:#X}", instruction)
                };

            }
            0xF => {
                match kk {
                    0x07 => {
                        //LD Vx, DT
                        self.vx[x as usize] = bus.get_delay_timer();
                        self.pc += 2;
                    }
                    0x0A => {
                        //LD Vx, K
                        if let Some(val) = bus.get_key_pressed() {
                            self.vx[x as usize] = val;
                            self.pc += 2;
                        }
                    }
                    0x15 => {
                        //LD DT, Vx
                        bus.set_delay_timer(self.vx[x as usize]);
                        self.pc += 2;
                    }
                    0x18 => {
                        //LD ST, Vx
                        self.sound = self.vx[x as usize];
                        self.pc += 2;
                    }
                    0x1E => {
                        //ADD I, Vx
                        self.i += self.vx[x as usize] as u16;
                        self.pc += 2;
                    }
                    0x29 => {
                        //LD F, Vx
                        self.i = self.vx[x as usize] as u16 * 5;
                        self.pc += 2;
                    }
                    0x33 => {
                        //LD B, Vx
                        let hundreds: u8 = (self.vx[x as usize] / 100) as u8;
                        let tens: u8 = ((self.vx[x as usize] % 100) / 10) as u8;
                        let ones: u8 = (self.vx[x as usize] % 10) as u8;
                        //write 100s digit
                        bus.ram_write_byte(self.i, hundreds);
                        //write 10s digit
                        bus.ram_write_byte(self.i+1, tens);
                        //write 1s digit
                        bus.ram_write_byte(self.i+2, ones);
                        self.pc += 2;
                    }
                    0x55 => {
                        //LD [I], Vx
                        for index in 0..x + 1 {
                            let value = self.vx[index as usize];
                            bus.ram_write_byte(self.i + (index as u16), value);
                        }
                        //self.i += x as u16 + 1;
                        self.pc += 2;
                    }
                    0x65 => {
                        //LD Vx, [I]
                        for index in 0..x + 1 {
                            let value = bus.ram_read_byte(self.i + (index as u16));
                            self.vx[0+index as usize] = value;
                        }
                        self.pc += 2;
                    }
                    _ => panic!("0xF instruction not recognized: {:#X}", instruction)
                };
            }
            _ => panic!("Unrecognized instruction {:#X}", instruction)
        }
    }

    fn debug_draw_sprite(&mut self, bus: &mut Bus, n: u8, x: u8, y: u8) {
        let mut should_set_vf = false;
        for sprite_y in 0..n {
            let b = bus.ram_read_byte(self.i + sprite_y as u16);
            if bus.debug_draw_byte(b, x, y + sprite_y) {
                should_set_vf = true;
            }
        }

        if should_set_vf {
            self.vx[0xf as usize] = 1;
        } else {
            self.vx[0xf as usize] = 0;
        }
    }
}