use std::process;
use crate::*;

pub struct ModRM {

    pub modval: u8,
    pub opecode: u8,
    pub reg_index: u8,
    pub rm: u8,
    pub sib: u8,
    pub disp8: i8,
    pub disp32: u32,
}

impl Default for ModRM {
    fn default () -> ModRM {
        ModRM {
            modval: 0,
            opecode: 0,
            reg_index: 0,
            rm: 0,
            sib: 0,
            disp8: 0,
            disp32: 0,
        }
    }
}

pub fn parse_modrm(emu: &mut Emulator) -> ModRM {
    let code = get_code8(emu, 0);
    let mut modrm = ModRM::default();
    modrm.modval = ((code & 0xc0) >> 6).try_into().unwrap();
    modrm.opecode = ((code & 0x38) >> 3).try_into().unwrap();
    modrm.reg_index = ((code & 0x38) >> 3).try_into().unwrap();
    modrm.rm = (code & 0x07).try_into().unwrap();

    emu.eip += 1;

    if modrm.modval != 3 && modrm.rm == 4 {
        modrm.sib = get_code8(emu, 0).try_into().unwrap();
        emu.eip += 1;
    }

    if (modrm.modval == 0 && modrm.rm == 5) || modrm.modval == 2 {
        modrm.disp32 = get_sign_code32(emu, 0).try_into().unwrap();
        emu.eip += 1;
    } else if modrm.modval == 1 {
        modrm.disp8 = get_sign_code32(emu, 0).try_into().unwrap();
        emu.eip += 1;
    } 
    return modrm;
}



