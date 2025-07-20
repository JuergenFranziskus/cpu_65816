use crate::core::Core;

pub mod core;
pub mod instruction;

pub struct Bus {
    pub addr: u16,
    pub bank: u8,
    pub data: u8,
    flags: u8,
}
impl Bus {
    pub fn irq(self) -> bool {
        self.flags & Self::IRQ != 0
    }
    pub fn nmi(self) -> bool {
        self.flags & Self::NMI != 0
    }
    pub fn rdy(self) -> bool {
        self.flags & Self::RDY != 0
    }
    pub fn res(self) -> bool {
        self.flags & Self::RES != 0
    }
    pub fn rw(self) -> bool {
        self.flags & Self::RW != 0
    }
    pub fn vda(self) -> bool {
        self.flags & Self::VDA != 0
    }
    pub fn vpa(self) -> bool {
        self.flags & Self::VPA != 0
    }

    pub fn set_irq(&mut self, to: bool) {
        self.flags &= !Self::IRQ;
        if to {
            self.flags |= Self::IRQ;
        }
    }
    pub fn set_nmi(&mut self, to: bool) {
        self.flags &= !Self::NMI;
        if to {
            self.flags |= Self::NMI;
        }
    }
    pub fn set_rdy(&mut self, to: bool) {
        self.flags &= !Self::RDY;
        if to {
            self.flags |= Self::RDY;
        }
    }
    pub fn set_res(&mut self, to: bool) {
        self.flags &= !Self::RES;
        if to {
            self.flags |= Self::RES;
        }
    }
    pub fn set_rw(&mut self, to: bool) {
        self.flags &= !Self::RW;
        if to {
            self.flags |= Self::RW;
        }
    }
    pub fn set_vda(&mut self, to: bool) {
        self.flags &= !Self::VDA;
        if to {
            self.flags |= Self::VDA;
        }
    }
    pub fn set_vpa(&mut self, to: bool) {
        self.flags &= !Self::VPA;
        if to {
            self.flags |= Self::VPA;
        }
    }

    const IRQ: u8 = 1;
    const NMI: u8 = 2;
    const RDY: u8 = 4;
    const RES: u8 = 8;
    const RW: u8 = 16;
    const VDA: u8 = 32;
    const VPA: u8 = 64;
}

pub struct Cpu65816 {
    core: Core,
}
