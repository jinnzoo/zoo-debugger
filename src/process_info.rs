use libc::user_regs_struct;
use core::ffi::c_void;
use std::fmt;

pub const STACKLEN: usize = 5;
pub const TEXTLEN: usize = 5;

pub struct ProcessInfo {
    regs: user_regs_struct,
    stack: [(*mut c_void, *mut c_void); STACKLEN],
    text: [(*mut c_void, *mut c_void); TEXTLEN],
}

impl ProcessInfo {
    pub fn new(regs: user_regs_struct, stack:[(*mut c_void, *mut c_void); STACKLEN], text:[(*mut c_void, *mut c_void); TEXTLEN]) -> Self {
        ProcessInfo {regs, stack, text}
    }
}

impl fmt::Display for ProcessInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=== regs ===\n");
        write!(f, "{:#x?}\n", self.regs);
        write!(f, "============\n");
        write!(f, "=== text ===\n");
        for (addr, value) in self.text.iter() {
            write!(f, "{:?}:{:?}\n", addr, value);
        }
        write!(f, "============\n");
        write!(f, "=== stack ===\n");
        for (addr, value) in self.stack.iter() {
            write!(f, "{:?}:{:?}\n", addr, value);
        }
        write!(f, "============\n")
    }
}

