mod lib;
use crate::Vec::new;

struct Emulator {
    register: [u32, Register::VariantCount],
    memory: vec<u8>,
    eip: u32,
    eflag: u32,
}

impl Emulator {
    fn new(size: usize, eip: u32, esp: u32) -> Emulator {
        let mut emu = Emulator {
            register: [0, variant_count],
            memory: vec![0; size],
            eip: eip,
            eflag: 0,
        };

        emu.register[Register::ESP as usize] = esp;
        emu
    }

    fn destruct_emu(&self) {
        for regi in Register::iter() {
            match self.register.get(regi as usize) {
            Some(value) => println!("{:?} = {:>08}", &regi, value),
            None => eprintln!("Invalid register: {:?}", regi),
        }
    }
    println!("EIP = {:>08x}", self.eip);
}


}
