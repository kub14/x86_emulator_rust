mod lib;
use std::process;
use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;

pub mod function;
pub mod modrm;
use function::*;

const MEMORY_SIZE: usize = 1024 * 1024;

const EAX: usize = 0;
const ECX: usize = 1;
const EDX: usize = 2;
const EBX: usize = 3;
const ESP: usize = 4;
const EBP: usize = 5;
const ESI: usize = 6;
const EDI: usize = 7;
const REGISTERS_COUNT: usize = 8;
const REGISTERS_NAME: [&str; 8] = ["EAX", "ECX", "EDX","EBX", "ESP","EBP", "ESI", "EDI"];

type InstFunc = fn(&mut Emulator);
type Insts = [InstFunc; 256];


pub struct Emulator {
    register: [u32; REGISTERS_COUNT],
    memory: Vec<u8>,
    eip: usize,
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


fn read_binary(emu: &mut Emulator, filename: &String) -> u64 {
    let path = Path::new(&filename);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,why.description()),
        Ok(file) => file,
    };
    let file_len = file.metadata().unwrap().len();

    let mut binary = vec::<u8>::new();
    match file.read_to_end(&mut binary) {
        Err(Why) => panic!("couldn't read {}: {}", display,why.description()),
        Ok(_) => println!("read file from {}\n", display),
    }

    emu.memory = vec![0; 0x7c00];
    emu.memory = extend(binary);
    return file_len
}

fn create_emu(eip: usize, esp: u32) -> Emulator {
    let mut register = [0; REGISTERS_COUNT];
    registers[ESP] = esp;
    return Emulator {
        register: registers,
        memory: Vec::new(),
        eflag: 0,
        eip: eip,
    }
}

fn dump_registers(emu: &mut Emulator) {
    println!("----- registers -----");
    for i in 0..REGISTERS_COUNT {
        println!("Current Index = No.{0} \n
        {1} = {1}", i,REGISTERS_NAME[i], get_register32(emu, 1));
    }
    println!("EIP = {}", emu.eip);
}

fn dump_stack(emu: &mut Emulator) {
    println!("----- stack -----");
    for i in 0..10 {
        let address = emu.register[ESP] - 4 * i;
        let value = get_memory32(emu, address.try_into().unwrap());
        println!("stack [{}]: {}", address, value);
    }
}

fn dump_eflag(emu: &mut Emulator) {
    println!("----- eflag -----");
    println!("carry: {}", emu.eflag & 1);

}



