use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Core {
    pub a: u16,
    pub d: u16,
    pub dbr: u8,
    pub e: bool,
    pub p: P,
    pub pbr: u8,
    pub pc: u16,
    pub s: u16,
    pub x: u16,
    pub y: u16,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct P(pub u8);
impl P {
    pub fn c(self) -> bool {
        self.0 & Self::C != 0
    }
    pub fn z(self) -> bool {
        self.0 & Self::Z != 0
    }
    pub fn i(self) -> bool {
        self.0 & Self::I != 0
    }
    pub fn d(self) -> bool {
        self.0 & Self::D != 0
    }
    pub fn x(self) -> bool {
        self.0 & Self::X != 0
    }
    pub fn m(self) -> bool {
        self.0 & Self::M != 0
    }
    pub fn v(self) -> bool {
        self.0 & Self::V != 0
    }
    pub fn n(self) -> bool {
        self.0 & Self::N != 0
    }

    pub fn set_c(&mut self, to: bool) {
        self.0 &= !Self::C;
        if to {
            self.0 |= Self::C
        }
    }
    pub fn set_z(&mut self, to: bool) {
        self.0 &= !Self::Z;
        if to {
            self.0 |= Self::Z
        }
    }
    pub fn set_i(&mut self, to: bool) {
        self.0 &= !Self::I;
        if to {
            self.0 |= Self::I
        }
    }
    pub fn set_d(&mut self, to: bool) {
        self.0 &= !Self::D;
        if to {
            self.0 |= Self::D
        }
    }
    pub fn set_x(&mut self, to: bool) {
        self.0 &= !Self::X;
        if to {
            self.0 |= Self::X
        }
    }
    pub fn set_m(&mut self, to: bool) {
        self.0 &= !Self::M;
        if to {
            self.0 |= Self::M
        }
    }
    pub fn set_v(&mut self, to: bool) {
        self.0 &= !Self::V;
        if to {
            self.0 |= Self::V
        }
    }
    pub fn set_n(&mut self, to: bool) {
        self.0 &= !Self::N;
        if to {
            self.0 |= Self::N
        }
    }

    pub fn with_c(mut self, to: bool) -> Self {
        self.set_c(to);
        self
    }
    pub fn with_z(mut self, to: bool) -> Self {
        self.set_z(to);
        self
    }
    pub fn with_i(mut self, to: bool) -> Self {
        self.set_i(to);
        self
    }
    pub fn with_d(mut self, to: bool) -> Self {
        self.set_d(to);
        self
    }
    pub fn with_x(mut self, to: bool) -> Self {
        self.set_x(to);
        self
    }
    pub fn with_m(mut self, to: bool) -> Self {
        self.set_m(to);
        self
    }
    pub fn with_v(mut self, to: bool) -> Self {
        self.set_v(to);
        self
    }
    pub fn with_n(mut self, to: bool) -> Self {
        self.set_n(to);
        self
    }

    const C: u8 = 1;
    const Z: u8 = 2;
    const I: u8 = 4;
    const D: u8 = 8;
    const X: u8 = 16;
    const M: u8 = 32;
    const V: u8 = 64;
    const N: u8 = 128;
}
impl Display for P {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = if self.c() { "C" } else { "-" };
        let z = if self.c() { "Z" } else { "-" };
        let i = if self.c() { "I" } else { "-" };
        let d = if self.c() { "D" } else { "-" };
        let x = if self.c() { "X" } else { "-" };
        let m = if self.c() { "M" } else { "-" };
        let v = if self.c() { "V" } else { "-" };
        let n = if self.c() { "N" } else { "-" };

        write!(f, "{n}{v}{m}{x}{d}{i}{z}{c}")
    }
}
