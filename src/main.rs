use std::fs::File;
use std::io::Read;
use crate::chip8::Chip8;

mod ram;
mod chip8;

fn main() {
    let mut file = File::open("roms/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    //println!("{:?}", data.to_vec());

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);
}